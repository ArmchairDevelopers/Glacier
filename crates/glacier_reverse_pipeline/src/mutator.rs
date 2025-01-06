use glacier_fs::db::partition::DatabasePartition;
use glacier_reflect::type_info::{TypeInfo, TypeObject};
use glacier_store::domain::DomainStore;
use glacier_util::guid::Guid;

use crate::pipeline::PipelineStorage;

pub mod blueprint;
pub mod class_info_asset;
pub mod level_description;
pub mod mesh_variation_db;
pub mod network_registry;
pub mod rvm_database;

#[derive(Debug)]
pub enum UnbuildAction {
    SaveSelf,
    SaveAsset(DatabasePartition),
    DeleteAsset(Guid),
}

#[derive(Debug)]
pub struct UnbuildResult {
    pub actions: Vec<UnbuildAction>,
}

impl UnbuildResult {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn save_self(&mut self) {
        self.actions.push(UnbuildAction::SaveSelf);
    }

    pub fn save_asset(&mut self, partition: DatabasePartition) {
        self.actions.push(UnbuildAction::SaveAsset(partition));
    }

    pub fn delete_asset(&mut self, partition_guid: Guid) {
        self.actions
            .push(UnbuildAction::DeleteAsset(partition_guid));
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PipelineMutationError {
    #[error("failed to read asset: {0}")]
    DbxReaderError(#[from] glacier_fs::dbx::reader::DbxReaderError),
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
        result: &mut UnbuildResult,
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
        result: &mut UnbuildResult,
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
        result: &mut UnbuildResult,
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
        result: &mut UnbuildResult,
    ) -> Result<(), PipelineMutationError>;

    async fn post_mutation(
        &self,
        domain: &DomainStore,
        storage: &PipelineStorage,
        result: &mut UnbuildResult,
    ) -> Result<(), PipelineMutationError> {
        Ok(())
    }
}
