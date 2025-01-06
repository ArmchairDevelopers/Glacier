use glacier_fs::{db::partition::DatabasePartition, util::trait_coercion::type_info_cast};
use glacier_reflect::type_info::{TypeInfo, TypeObject};
use glacier_reflect_swbf2::entity::{
    Blueprint, NetworkRegistryAsset, BLUEPRINT_TYPE_INFO, NETWORKREGISTRYASSET_TYPE_INFO,
};
use glacier_store::domain::DomainStore;

use super::{PipelineAssetMutator, PipelineMutationError, UnbuildResult};

pub struct BlueprintMutator;

#[async_trait::async_trait]
impl PipelineAssetMutator for BlueprintMutator {
    fn asset_type(&self) -> &'static TypeInfo {
        BLUEPRINT_TYPE_INFO
    }

    async fn mutate(
        &self,
        domain: &DomainStore,
        partition: &mut DatabasePartition,
        result: &mut UnbuildResult,
    ) -> Result<(), PipelineMutationError> {
        let mut primary = partition
            .primary_instance()
            .expect("Asset has no primary instance")
            .lock()
            .await;

        let asset = type_info_cast::<Blueprint>(&mut *primary, BLUEPRINT_TYPE_INFO).unwrap();

        if let Some(schematics) = &asset.schematics {
            result.save_self();

            let schematics = schematics.lock().await;
            let partition_guid = schematics.data_container_core().unwrap().partition_guid;

            result.delete_asset(partition_guid);
        };

        asset.schematics = None;
        Ok(())
    }
}
