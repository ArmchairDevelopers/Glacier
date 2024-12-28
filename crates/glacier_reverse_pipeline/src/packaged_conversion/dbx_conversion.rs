use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{atomic::AtomicU32, Arc},
};

use glacier_fs::{
    db::partition::DatabasePartition,
    ebx::partition::{EbxError, EbxPartitionHeader, EbxPartitionImportLoader, EbxPartitionReader},
    fb::{
        cas::CAS_MAX_CHUNK_SIZE, manifest::ManifestFileInfo, ConverterContext, FrostbiteGameData,
    },
};
use glacier_reflect::{type_info::LockedTypeObject, type_registry::TypeRegistry};
use glacier_reflect_swbf2::core::DATACONTAINER_TYPE_INFO;
use glacier_store::index::asset_index::{DomainAssetIndex, DomainAssetIndexEntry};
use glacier_util::guid::Guid;
use serde::Serialize;
use tokio::{
    fs,
    sync::{Mutex, Semaphore},
};
use tracing::{error, info, warn};

use super::PackagedConversionContext;

#[derive(Clone)]
pub struct PackagedConversionEbxPartitionImportLoader {
    index: Arc<DomainAssetIndex>,

    registry: Arc<TypeRegistry>,
    context: ConverterContext,
    partition_to_file: Arc<HashMap<Guid, ManifestFileInfo>>,

    loaded_instances: HashMap<(Guid, Guid), LockedTypeObject>,
}

impl PackagedConversionEbxPartitionImportLoader {
    pub fn new(
        index: Arc<DomainAssetIndex>,
        registry: Arc<TypeRegistry>,
        context: ConverterContext,
        partition_to_file: Arc<HashMap<Guid, ManifestFileInfo>>,
    ) -> Self {
        Self {
            index,
            registry,
            context,
            partition_to_file,
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

        let data = self.index.by_partition_guid(&partition_guid)?;
        let partition = DatabasePartition::new_empty(data.name.to_owned(), partition_guid);
        let instance = partition
            .create_instance_with_id(instance_guid, DATACONTAINER_TYPE_INFO)
            .await
            .unwrap();
        self.loaded_instances
            .insert((partition_guid, instance_guid), instance.clone());
        Some(instance)
    }
}

async fn convert_to_dbx(base_path: PathBuf, registry: &TypeRegistry, partition: DatabasePartition) {
    let path = base_path
        .join("Source")
        .join(partition.name())
        .with_extension("dbx");

    tokio::fs::create_dir_all(&path.parent().unwrap())
        .await
        .unwrap();

    let mut dbx_writer = glacier_fs::dbx::writer::DbxPartitionWriter::new(&partition, &registry);

    let mut writer = tokio::fs::File::create(path).await.unwrap();
    dbx_writer.write(&mut writer).await.unwrap();
}

pub(crate) async fn convert_ebx(
    ctx: &PackagedConversionContext,
    type_registry: &Arc<TypeRegistry>,
    data: &Arc<FrostbiteGameData>,
) {
    let max_concurrent_jobs = 5000; // Adjust this limit based on your system's capacity
    let semaphore = Arc::new(Semaphore::new(max_concurrent_jobs));

    let mut handles = Vec::new();

    let mut parse_files = Vec::new();
    let mut partition_to_file = HashMap::new();

    info!("Loading indexed partitions...");

    let str = fs::read(ctx.state_data_path().await.join("indexed_partitions.json"))
        .await
        .expect("Failed to write indexed partitions");
    let asset_index = DomainAssetIndex::load(String::from_utf8(str).unwrap()).unwrap();

    info!("Creating partition file map...");

    for bundle in &data.bundles {
        for entry in &bundle.ebx_entries {
            match asset_index.by_name(&entry.name) {
                Some(data) => {
                    if partition_to_file
                        .insert(data.partition, entry.file.clone())
                        .is_none()
                    {
                        parse_files.push((entry.name.to_owned(), entry.file.clone()));
                    }
                }
                None => {
                    warn!(
                        "CONV-1 Partition not found in asset index: {:?}",
                        entry.name
                    );
                }
            }
        }
    }

    info!("Starting conversion...");

    let asset_index = Arc::new(asset_index);
    let partition_to_file = Arc::new(partition_to_file);

    let mut i = 0;
    let len = parse_files.len();

    for entry in parse_files {
        if i % 5000 == 0 {
            info!("Processed {}/{} partitions", i, len,);
        }

        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let cloned_ctx = data.ctx.clone();
        let cloned_registry = type_registry.clone();
        let cloned_asset_index = asset_index.clone();
        let cloned_partition_to_file = partition_to_file.clone();
        let cloned_output_path = ctx.output_data_path.clone();
        handles.push(tokio::spawn(async move {
            let result = entry.1.read_data(&cloned_ctx, None).await;

            match result {
                Ok(data) => {
                    let mut loader = PackagedConversionEbxPartitionImportLoader::new(
                        cloned_asset_index,
                        cloned_registry.clone(),
                        cloned_ctx,
                        cloned_partition_to_file,
                    );

                    let mut reader =
                        EbxPartitionReader::new(entry.0, &cloned_registry, Some(&mut loader));

                    let result = reader.read(data).await;
                    if let Err(err) = result {
                        //error!("CONV-2 Error reading EBX: {:?}", err);
                        return;
                    }

                    let partition = reader.finalize();
                    convert_to_dbx(cloned_output_path, &cloned_registry, partition).await;
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
    info!("Conversion finished!");
}
