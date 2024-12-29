use std::{path::PathBuf, sync::Arc};

use glacier_fs::{
    db::partition::DatabasePartition,
    dbx::reader::{DbxPartitionImportLoader, DbxPartitionReader, DbxReaderError},
};
use glacier_reflect::{type_info::LockedTypeObject, type_registry::TypeRegistry};
use glacier_reflect_swbf2::core::DATACONTAINER_TYPE_INFO;
use glacier_util::guid::Guid;
use tokio::fs;

use crate::index::asset_index::DomainAssetIndex;

pub struct StubDbxPartitionImportLoader {
    pub index: Arc<DomainAssetIndex>,
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
        let data = self.index.by_partition_guid(&partition_guid)?;
        let partition = DatabasePartition::new_empty(data.name.to_owned(), partition_guid);
        let instance = partition
            .create_instance_with_id(instance_guid, DATACONTAINER_TYPE_INFO)
            .await
            .unwrap();
        Some(instance)
    }
}

pub struct DomainStore {
    type_registry: Arc<TypeRegistry>,
    asset_index: Arc<DomainAssetIndex>,
    base_path: PathBuf,
}

impl DomainStore {
    pub fn new(
        type_registry: Arc<TypeRegistry>,
        asset_index: Arc<DomainAssetIndex>,
        base_path: PathBuf,
    ) -> Self {
        Self {
            type_registry,
            asset_index,
            base_path,
        }
    }

    pub async fn load_asset(&self, asset_path: &str) -> Result<DatabasePartition, DbxReaderError> {
        let path = self.base_path.join(asset_path).with_extension("dbx");
        if !path.exists() {
            return Err(DbxReaderError::FileNotFound);
        }

        let mut loader = StubDbxPartitionImportLoader {
            index: self.asset_index.clone(),
        };

        let mut reader = DbxPartitionReader::new(asset_path.to_owned(), &self.type_registry, Some(&mut loader));

        let file = fs::File::open(path).await?;
        reader.read(file).await?;

        Ok(reader.finalize().await)
    }

    pub fn index(&self) -> &Arc<DomainAssetIndex> {
        &self.asset_index
    }
}
