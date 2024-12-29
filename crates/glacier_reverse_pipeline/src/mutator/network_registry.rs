use glacier_reflect::type_info::{TypeInfo, TypeObject};
use glacier_reflect_swbf2::entity::{
    NetworkRegistryAsset, NETWORKREGISTRYASSET_TYPE_INFO,
};

use super::{PipelineMutationError, PipelineAssetMutator, UnbuildResult};

pub struct NetworkRegistryMutator;

#[async_trait::async_trait]
impl PipelineAssetMutator for NetworkRegistryMutator {
    fn asset_type(&self) -> &'static TypeInfo {
        NETWORKREGISTRYASSET_TYPE_INFO
    }

    async fn mutate(
        &self,
        asset: &mut dyn TypeObject,
        result: &mut UnbuildResult,
    ) -> Result<(), PipelineMutationError> {
        let asset = asset
            .as_any()
            .downcast_ref::<NetworkRegistryAsset>()
            .map_or_else(
                || {
                    Err(PipelineMutationError::Custom(
                        "Asset is not a NetworkRegistryAsset".to_string(),
                    ))
                },
                |asset| Ok(asset),
            )?;

        let partition = asset.data_container_core().unwrap().partition_guid;
        result.delete_asset(partition);

        Ok(())
    }
}
