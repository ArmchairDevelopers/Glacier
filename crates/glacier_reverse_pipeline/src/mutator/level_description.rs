use glacier_fs::db::partition::DatabasePartition;
use glacier_reflect::type_info::{TypeInfo, TypeObject};
use glacier_reflect_swbf2::{core::CLASSINFOASSET_TYPE_INFO, gameplay_sim::LEVELDESCRIPTIONASSET_TYPE_INFO};
use glacier_store::domain::DomainStore;

use super::{PipelineAssetMutator, PipelineMutationError, UnbuildResult};

pub struct LevelDescriptionMutator;

#[async_trait::async_trait]
impl PipelineAssetMutator for LevelDescriptionMutator {
    fn asset_type(&self) -> &'static TypeInfo {
        LEVELDESCRIPTIONASSET_TYPE_INFO
    }

    async fn mutate(
        &self,
        domain: &DomainStore,
        partition: &mut DatabasePartition,
        result: &mut UnbuildResult,
    ) -> Result<(), PipelineMutationError> {
        result.delete_asset(*partition.guid());
        Ok(())
    }
}
