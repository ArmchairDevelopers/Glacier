use glacier_fs::db::partition::DatabasePartition;
use glacier_fs::dbx::reader::DbxReaderError;
use glacier_reflect::type_info::{TypeInfo, TypeObject};
use glacier_store::domain::{DomainStore, DomainStoreError};
use glacier_util::guid::Guid;

use crate::pipeline::PipelineStorage;

pub mod blueprint;
pub mod class_info_asset;
pub mod level_description;
pub mod mesh_variation_db;
pub mod network_registry;
pub mod rvm_database;

#[derive(Debug, Hash)]
pub enum BuildAction {
    SaveSelf,
    SaveAsset(DatabasePartition),
    DeleteAsset(Guid),
}

#[derive(Debug, Hash)]
pub struct BuildResult {
    pub actions: Vec<BuildAction>,
}

impl BuildResult {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn save_self(&mut self) {
        self.actions.push(BuildAction::SaveSelf);
    }

    pub fn save_asset(&mut self, partition: DatabasePartition) {
        self.actions.push(BuildAction::SaveAsset(partition));
    }

    pub fn delete_asset(&mut self, partition_guid: Guid) {
        self.actions
            .push(BuildAction::DeleteAsset(partition_guid));
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PipelineMutationError {
    #[error(transparent)]
    IoError(#[from] tokio::io::Error),
    #[error("failed to read asset: {0}")]
    DbxReaderError(#[from] DbxReaderError),
    #[error("domain store error: {0}")]
    DomainStoreError(#[from] DomainStoreError),
    #[error("{0}")]
    Custom(String),
}

#[async_trait::async_trait]
pub trait PipelineAssetMutator: Sync + Send {
    /// Whether imports in assets should be resolved.
    fn shallow_load(&self) -> bool {
        true
    }

    fn max_concurrent_jobs(&self) -> usize {
        50
    }

    fn asset_type(&self) -> &'static TypeInfo;

    async fn mutate(
        &self,
        domain: &DomainStore,
        partition: &mut DatabasePartition,
        result: &mut BuildResult,
    ) -> Result<(), PipelineMutationError>;
}

#[async_trait::async_trait]
pub trait PipelineDataContainerPolicyMutator {
    /// Whether imports in assets should be resolved.
    fn shallow_load(&self) -> bool {
        true
    }

    fn force_synchronous(&self) -> bool {
        false
    }

    fn container_type(&self) -> &'static TypeInfo;

    async fn mutate(
        &self,
        domain: &DomainStore,
        partition: &mut DatabasePartition,
        result: &mut BuildResult,
    ) -> Result<(), PipelineMutationError>;
}

#[async_trait::async_trait]
pub trait PipelineResourceMutator: Sync + Send {
    fn max_concurrent_jobs(&self) -> usize {
        1
    }

    fn resource_type_name(&self) -> &'static str;

    async fn pre_mutation(
        &self,
        domain: &DomainStore,
        storage: &PipelineStorage,
        result: &mut BuildResult,
    ) -> Result<(), PipelineMutationError> {
        Ok(())
    }

    async fn mutate(
        &self,
        domain: &DomainStore,
        storage: PipelineStorage,
        res_name: &str,
        res_id: u64,
        res_meta: &[u8],
        res_data: &[u8],
        result: &mut BuildResult,
    ) -> Result<(), PipelineMutationError>;

    async fn post_mutation(
        &self,
        domain: &DomainStore,
        storage: &PipelineStorage,
        result: &mut BuildResult,
    ) -> Result<(), PipelineMutationError> {
        Ok(())
    }
}
