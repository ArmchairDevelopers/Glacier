use glacier_fs::db::partition::DatabasePartition;
use glacier_reflect::type_info::{TypeInfo, TypeObject};
use glacier_reflect_swbf2::entity::{NetworkRegistryAsset, NETWORKREGISTRYASSET_TYPE_INFO};
use glacier_store::domain::DomainStore;

use super::{PipelineAssetMutator, PipelineMutationError, UnbuildResult};

pub struct NetworkRegistryMutator;

#[async_trait::async_trait]
impl PipelineAssetMutator for NetworkRegistryMutator {
    fn asset_type(&self) -> &'static TypeInfo {
        NETWORKREGISTRYASSET_TYPE_INFO
    }

    async fn mutate(
        &self,
        domain: &DomainStore,
        partition: &mut DatabasePartition,
        result: &mut UnbuildResult,
    ) -> Result<(), PipelineMutationError> {
        // let primary = partition
        //     .primary_instance()
        //     .expect("Asset has no primary instance")
        //     .lock()
        //     .await;

        // let asset = primary
        //     .as_any()
        //     .downcast_ref::<NetworkRegistryAsset>()
        //     .map_or_else(
        //         || {
        //             Err(PipelineMutationError::Custom(
        //                 "Asset is not a NetworkRegistryAsset".to_string(),
        //             ))
        //         },
        //         |asset| Ok(asset),
        //     )?;

        // let partition = asset.data_container_core().unwrap().partition_guid;
        // result.delete_asset(partition);

        result.delete_asset(*partition.guid());
        Ok(())
    }
}
