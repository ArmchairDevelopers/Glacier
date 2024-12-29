use glacier_reflect::type_info::{TypeInfo, TypeObject};
use glacier_util::guid::Guid;

pub mod network_registry;

pub enum UnbuildAction {
    DeleteAsset(Guid),
}

pub struct UnbuildResult {
    pub actions: Vec<UnbuildAction>,
}

impl UnbuildResult {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn delete_asset(&mut self, partition_guid: Guid) {
        self.actions.push(UnbuildAction::DeleteAsset(partition_guid));
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PipelineMutationError {
    #[error("{0}")]
    Custom(String),
}

#[async_trait::async_trait]
pub trait PipelineAssetMutator {
    /// Whether imports in assets should be resolved.
    fn shallow_load(&self) -> bool {
        true
    }

    fn force_synchronous(&self) -> bool {
        false
    }

    fn asset_type(&self) -> &'static TypeInfo;

    async fn mutate(&self, asset: &mut dyn TypeObject, result: &mut UnbuildResult) -> Result<(), PipelineMutationError>;
}
