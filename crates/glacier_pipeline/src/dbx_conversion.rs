use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{atomic::AtomicU32, Arc},
};

use glacier_fs::{
    db::partition::DatabasePartition,
    dbx::writer::DbxPartitionWriter,
    ebx::partition::{EbxPartitionImportLoader, EbxPartitionReader},
    fb::FrostbiteGameData,
};
use glacier_reflect::{type_info::LockedTypeObject, type_registry::TypeRegistry};
use glacier_reflect_swbf2::core::DATACONTAINER_TYPE_INFO;
use glacier_store::index::asset_index::{AssetIndexDbxWriterImportResolver, DomainAssetIndex};
use glacier_util::guid::Guid;
use tokio::{
    fs,
    sync::{RwLock, Semaphore},
};
use tracing::{error, info};

use super::PackagedConversionContext;

#[derive(Clone)]
pub struct PackagedConversionEbxPartitionImportLoader {
    index: Arc<RwLock<DomainAssetIndex>>,
    loaded_instances: HashMap<(Guid, Guid), LockedTypeObject>,
}

impl PackagedConversionEbxPartitionImportLoader {
    pub fn new(index: Arc<RwLock<DomainAssetIndex>>) -> Self {
        Self {
            index,
            loaded_instances: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl EbxPartitionImportLoader for PackagedConversionEbxPartitionImportLoader {
    async fn store_instance(
        &mut self,
        partition_guid: Guid,
        instance_guid: Guid,
        instance: LockedTypeObject,
    ) {
        self.loaded_instances
            .insert((partition_guid, instance_guid), instance);
    }

    async fn load_partition(
        &mut self,
        partition_guid: Guid,
        instance_guid: Guid,
    ) -> Option<LockedTypeObject> {
        if let Some(instance) = self.loaded_instances.get(&(partition_guid, instance_guid)) {
            return Some(instance.clone());
        }

        let name = self
            .index
            .read()
            .await
            .get_name_by_partition_guid(&partition_guid)?
            .to_owned();

        let mut partition = DatabasePartition::new_empty(name, partition_guid);
        let instance = partition
            .create_instance_with_id(instance_guid, DATACONTAINER_TYPE_INFO)
            .await
            .unwrap();

        self.loaded_instances
            .insert((partition_guid, instance_guid), instance.clone());

        Some(instance)
    }
}

async fn convert_to_dbx(
    base_path: PathBuf,
    registry: &TypeRegistry,
    index: Arc<RwLock<DomainAssetIndex>>,
    partition: DatabasePartition,
) {
    let path = base_path
        .join("Source")
        .join(partition.name())
        .with_extension("dbx");

    fs::create_dir_all(&path.parent().unwrap()).await.unwrap();

    let import_resolver = AssetIndexDbxWriterImportResolver::new(index);
    let mut dbx_writer = DbxPartitionWriter::new(&partition, &registry, &import_resolver);

    let mut writer = fs::File::create(path).await.unwrap();
    dbx_writer.write(&mut writer).await.unwrap();
}

pub(crate) async fn convert_ebx(
    ctx: &PackagedConversionContext,
    asset_index: &Arc<RwLock<DomainAssetIndex>>,
    type_registry: &Arc<TypeRegistry>,
    data: &Arc<FrostbiteGameData>,
) {
    let max_concurrent_jobs = 5000;
    let semaphore = Arc::new(Semaphore::new(max_concurrent_jobs));

    let mut handles = Vec::new();

    info!("Creating partition file map...");

    let mut unique_partitions = HashSet::new();
    let mut parse_files = Vec::new();

    {
        let asset_index = asset_index.read().await;
        for bundle in &data.bundles {
            for entry in &bundle.ebx_entries {
                match asset_index.get_by_name(&entry.name) {
                    Some(data) => {
                        if unique_partitions.insert(data.partition) {
                            parse_files.push((entry.name.to_owned(), entry.file.clone()));
                        }
                    }
                    None => {}
                }
            }
        }
    }

    info!("Starting conversion...");

    let error_count = Arc::new(AtomicU32::new(0));

    let mut i = 0;
    let len = parse_files.len();

    for entry in parse_files {
        if i % 5000 == 0 {
            info!("Progress: {}%", i as f32 / len as f32 * 100.0);
        }

        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let cloned_ctx = data.ctx.clone();
        let cloned_registry = type_registry.clone();
        let cloned_asset_index = asset_index.clone();
        let cloned_output_path = ctx.output_data_path.clone();
        let cloned_error_count = error_count.clone();
        handles.push(tokio::spawn(async move {
            let result = entry.1.read_data(&cloned_ctx, None).await;

            match result {
                Ok(data) => {
                    let mut loader =
                        PackagedConversionEbxPartitionImportLoader::new(cloned_asset_index.clone());

                    let mut reader =
                        EbxPartitionReader::new(entry.0, &cloned_registry, Some(&mut loader));

                    let result = reader.read(data).await;
                    if let Err(err) = result {
                        error!("CONV-2 Error reading EBX: {:?}", err);
                        cloned_error_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        return;
                    }

                    let partition = reader.finalize();
                    convert_to_dbx(
                        cloned_output_path,
                        &cloned_registry,
                        cloned_asset_index,
                        partition,
                    )
                    .await;
                }
                Err(err) => {
                    error!("CONV-3 Error reading EBX: {:?}", err);
                }
            }

            drop(permit);
        }));

        i += 1;
    }

    info!("Waiting for conversion to finish...");
    futures::future::join_all(handles).await;
    info!(
        "Converted {} EBX assets to DBX with {} error(s)",
        len,
        error_count.load(std::sync::atomic::Ordering::SeqCst)
    );
}
