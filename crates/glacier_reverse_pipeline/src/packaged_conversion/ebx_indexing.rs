use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{atomic::AtomicU32, Arc},
};

use glacier_fs::{
    db::partition::DatabasePartition,
    ebx::partition::{EbxError, EbxPartitionHeader, EbxPartitionReader},
    fb::{cas::CAS_MAX_CHUNK_SIZE, FrostbiteGameData},
};
use glacier_reflect::type_registry::TypeRegistry;
use glacier_store::index::asset_index::DomainAssetIndexEntry;
use glacier_util::guid::Guid;
use serde::Serialize;
use tokio::{
    fs,
    sync::{Mutex, Semaphore},
};

use super::PackagedConversionContext;

#[derive(Serialize)]
struct EbxIndexEntry {
    partition_guid: String,
    primary_type: String,
    bundles: Vec<u32>,
    imports: Vec<String>, // Partition IDs
}

async fn convert_to_dbx(registry: &TypeRegistry, partition: DatabasePartition) {
    let path = PathBuf::from("SourceData/Source")
        .join(partition.name())
        .with_extension("dbx");

    //println!("Writing DBX to {:?}", path);
    tokio::fs::create_dir_all(&path.parent().unwrap())
        .await
        .unwrap();

    let mut dbx_writer = glacier_fs::dbx::writer::DbxPartitionWriter::new(&partition, &registry);

    let mut writer = tokio::fs::File::create(path).await.unwrap();
    dbx_writer.write(&mut writer).await.unwrap();
}

pub(crate) async fn index_ebx(
    ctx: &PackagedConversionContext,
    type_registry: &Arc<TypeRegistry>,
    data: &Arc<FrostbiteGameData>,
) {
    let indexed_partitions = Arc::new(Mutex::new(HashMap::<String, DomainAssetIndexEntry>::new()));
    let noretail_fields = Arc::new(Mutex::new(HashSet::new()));

    let max_concurrent_jobs = 2000; // Adjust this limit based on your system's capacity
    let semaphore = Arc::new(Semaphore::new(max_concurrent_jobs));

    let mut handles = Vec::new();

    for bundle in &data.bundles {
        // println!(
        //     "Starting bundle with {} indexed partitions",
        //     unique_indexed_partitions.len()
        // );

        // {
        //     let indexed_partitions = indexed_partitions.lock().await;
        //     println!(
        //         "Indexed {} assets",
        //         indexed_partitions.len()
        //     );
        // }

        for entry in &bundle.ebx_entries {
            {
                let mut indexed_partitions = indexed_partitions.lock().await;
                if let Some(entry) = indexed_partitions.get_mut(&entry.name) {
                    entry.bundles.push(bundle.hash);
                    continue;
                } else {
                    let index_entry = DomainAssetIndexEntry {
                        name: entry.name.to_owned(),
                        partition: Guid::default(),
                        instances: Vec::new(),
                        primary_type: String::new(),
                        bundles: vec![bundle.hash],
                        imports: Vec::new(),
                    };

                    indexed_partitions.insert(entry.name.to_owned(), index_entry);
                }
            }

            let permit = semaphore.clone().acquire_owned().await.unwrap();

            let cloned_entry = entry.clone();
            let cloned_ctx = data.ctx.clone();
            let cloned_registry = type_registry.clone();
            let cloned_fields = noretail_fields.clone();
            let cloned_indexed_partitions = indexed_partitions.clone();
            handles.push(tokio::spawn(async move {
                let result = cloned_entry.file.read_data(&cloned_ctx, None).await;

                match result {
                    Ok(data) => {
                        let mut reader = EbxPartitionReader::new(
                            cloned_entry.name.to_owned(),
                            &cloned_registry,
                            None,
                        );
                        reader.layout_only();

                        let result = reader.read(data).await;
                        if let Err(err) = result {
                            //eprintln!("Error reading EBX: {:?}", err);

                            if let EbxError::FieldNotFound(name, type_name, enclosing_type_name) =
                                err
                            {
                                let mut fields = cloned_fields.lock().await;
                                fields.insert((enclosing_type_name, type_name, name));
                            }
                            return;
                        }

                        let imports = reader
                            .import_entries()
                            .into_iter()
                            .map(|x| x.partition_guid)
                            .collect();
                        let partition = reader.finalize();

                        let mut indexed_partitions = cloned_indexed_partitions.lock().await;
                        let index_entry = indexed_partitions.get_mut(&cloned_entry.name).unwrap();

                        index_entry.partition = *partition.guid();

                        let mut instances = Vec::new();
                        for instance in partition.instances() {
                            instances.push(
                                instance
                                    .lock()
                                    .await
                                    .data_container_core()
                                    .unwrap()
                                    .instance_guid,
                            );
                        }

                        index_entry.instances = instances;

                        // index_entry.primary_type = partition
                        //     .primary_instance()
                        //     .unwrap()
                        //     .lock()
                        //     .await
                        //     .type_info()
                        //     .name
                        //     .to_owned();

                        index_entry.imports = imports;

                        //convert_to_dbx(&cloned_registry, partition).await;
                    }
                    Err(err) => {
                        eprintln!("Error reading EBX: {:?}", err);
                    }
                }

                drop(permit);
            }));
        }
    }

    futures::future::join_all(handles).await;

    let indexed_partitions = indexed_partitions.lock().await;
    let json = serde_json::to_string(
        &indexed_partitions
            .values()
            .into_iter()
            .collect::<Vec<&DomainAssetIndexEntry>>(),
    )
    .unwrap();
    fs::write(
        ctx.state_data_path().await.join("indexed_partitions.json"),
        json,
    )
    .await
    .expect("Failed to write indexed partitions");
}
