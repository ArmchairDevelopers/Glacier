use std::{io::Cursor, path::PathBuf, sync::Arc};

use glacier_fs::{
    db::partition::DatabasePartition,
    dbx::{
        reader::{DbxPartitionImportLoader, DbxPartitionReader, DbxReaderError},
        writer::DbxPartitionWriter,
    },
};
use glacier_reflect::{type_info::LockedTypeObject, type_registry::TypeRegistry};
use glacier_reflect_swbf2::core::DATACONTAINER_TYPE_INFO;
use glacier_util::guid::Guid;
use tokio::{
    fs,
    sync::{RwLock, RwLockReadGuard},
};

use crate::index::asset_index::{
    AssetIndexDbxWriterImportResolver, DomainAssetIndex, DomainAssetIndexEntry,
    DomainAssetIndexInstance,
};

pub struct StubDbxPartitionImportLoader {
    pub index: Arc<RwLock<DomainAssetIndex>>,
}

#[async_trait::async_trait]
impl DbxPartitionImportLoader for StubDbxPartitionImportLoader {
    async fn store_instance(
        &mut self,
        _partition_guid: Guid,
        _instance_guid: Guid,
        _instance: LockedTypeObject,
    ) {
    }

    async fn load_partition(
        &mut self,
        partition_guid: Guid,
        instance_guid: Guid,
    ) -> Option<LockedTypeObject> {
        let index = self.index.read().await;
        let data = index.get_by_partition_guid(&partition_guid)?;

        let mut partition = DatabasePartition::new_empty(data.name.to_owned(), partition_guid);
        let instance = partition
            .create_instance_with_id(instance_guid, DATACONTAINER_TYPE_INFO)
            .await
            .unwrap();
        Some(instance)
    }
}

pub struct DomainStore {
    type_registry: Arc<TypeRegistry>,
    asset_index: Arc<RwLock<DomainAssetIndex>>,
    base_path: PathBuf,
    source_folder: String,
}

impl DomainStore {
    pub fn new(
        type_registry: Arc<TypeRegistry>,
        asset_index: Arc<RwLock<DomainAssetIndex>>,
        base_path: PathBuf,
        domain_folder: &str,
    ) -> Self {
        Self {
            type_registry,
            asset_index,
            base_path,
            source_folder: domain_folder.to_owned(),
        }
    }

    pub async fn load(
        type_registry: Arc<TypeRegistry>,
        base_path: PathBuf,
        domain_folder: &str,
    ) -> Result<Self, tokio::io::Error> {
        let index_path = base_path.join(".state/partition_index.json");
        let index = if index_path.exists() {
            let data = fs::read(index_path).await?;
            Arc::new(RwLock::new(
                DomainAssetIndex::load(String::from_utf8(data).unwrap()).unwrap(),
            ))
        } else {
            Arc::new(RwLock::new(DomainAssetIndex::new()))
        };

        Ok(Self::new(type_registry, index, base_path, domain_folder))
    }

    fn source_path(&self) -> PathBuf {
        self.base_path.join(&self.source_folder)
    }

    pub async fn load_asset(
        &self,
        asset_path: impl Into<String>,
    ) -> Result<DatabasePartition, DbxReaderError> {
        let path_str = asset_path.into();

        let path = self.source_path().join(&path_str).with_extension("dbx");
        if !path.exists() {
            return Err(DbxReaderError::FileNotFound);
        }

        let mut loader = StubDbxPartitionImportLoader {
            index: self.asset_index.clone(),
        };

        let mut reader = DbxPartitionReader::new(path_str, &self.type_registry, Some(&mut loader));

        let file = fs::read(path).await?;
        reader.read(Cursor::new(file)).await?;

        Ok(reader.finalize().await)
    }

    pub async fn load_asset_by_guid(
        &self,
        guid: &Guid,
    ) -> Result<DatabasePartition, DbxReaderError> {
        let name = {
            let index = self.asset_index.read().await;
            let data = index
                .get_by_partition_guid(guid)
                .ok_or(DbxReaderError::FileNotFound)?;
            data.name.to_owned()
        };
        
        self.load_asset(name).await
    }

    pub async fn load_asset_from_stub_instance(
        &self,
        stub: &LockedTypeObject,
    ) -> Result<DatabasePartition, DbxReaderError> {
        let instance = stub.lock().await;
        let data_container = instance.data_container_core().unwrap();
        self.load_asset_by_guid(&data_container.partition_guid)
            .await
    }

    pub async fn save_asset(&self, asset: &DatabasePartition) -> Result<(), tokio::io::Error> {
        let path = self.source_path().join(asset.name()).with_extension("dbx");
        fs::create_dir_all(&path.parent().unwrap()).await?;

        let import_resolver = AssetIndexDbxWriterImportResolver::new(self.asset_index.clone());
        let dbx_writer = DbxPartitionWriter::new(&asset, &self.type_registry, &import_resolver);

        let mut writer = fs::File::create(path).await?;
        dbx_writer.write(&mut writer).await?;

        let imports = import_resolver.imported_partitions();

        let entry = {
            let asset_index = self.asset_index.read().await;
            match asset_index.get_by_partition_guid(asset.guid()) {
                Some(entry) => {
                    let mut entry = entry.clone();
                    entry.imports.clear();
                    entry.imports.extend(imports);
                    entry
                }
                None => {
                    let mut instances = Vec::new();
                    for instance in asset.instances() {
                        let instance = instance.lock().await;

                        instances.push(DomainAssetIndexInstance {
                            guid: instance.data_container_core().unwrap().instance_guid,
                            type_hash: instance.type_info().name_hash,
                        });
                    }

                    DomainAssetIndexEntry {
                        name: asset.name().to_owned(),
                        partition: *asset.guid(),
                        primary_type_hash: instances[0].type_hash,
                        instances,
                        bundles: Vec::new(),
                        imports,
                    }
                }
            }
        };

        let mut asset_index = self.asset_index.write().await;
        asset_index.upsert_entry(entry);

        Ok(())
    }

    pub async fn delete_asset(&self, asset_path: &str) -> Result<(), std::io::Error> {
        let path = self.source_path().join(asset_path).with_extension("dbx");
        fs::remove_file(path).await?;

        self.asset_index
            .write()
            .await
            .delete_entry_by_name(asset_path);

        Ok(())
    }

    pub fn type_registry(&self) -> &Arc<TypeRegistry> {
        &self.type_registry
    }

    pub fn index(&self) -> &Arc<RwLock<DomainAssetIndex>> {
        &self.asset_index
    }

    pub async fn index_read(&self) -> RwLockReadGuard<DomainAssetIndex> {
        self.asset_index.read().await
    }

    pub async fn save_asset_index(&self) -> Result<(), tokio::io::Error> {
        let path = self.base_path.join(".state/partition_index.json");
        let index = self.asset_index.read().await;
        let data = serde_json::to_string(&index.values())?;
        fs::write(path, &data).await
    }
}
