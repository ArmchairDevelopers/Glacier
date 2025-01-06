use std::mem;

use glacier_fs::db::partition::DatabasePartition;
use glacier_reflect::type_info::{TypeInfo, TypeObject};
use glacier_reflect_ext::util::get_dc_instance_guid;
use glacier_reflect_swbf2::{
    entity::{NetworkRegistryAsset, NETWORKREGISTRYASSET_TYPE_INFO},
    render::{
        MeshMaterial, MeshVariationDatabase, MeshVariationDatabaseEntry,
        MeshVariationDatabaseMaterial, MESHVARIATIONDATABASE_TYPE_INFO,
    },
};
use glacier_store::domain::DomainStore;
use tracing::warn;

use super::{PipelineAssetMutator, PipelineMutationError, UnbuildResult};

pub struct MeshVariationDatabaseMutator;

#[async_trait::async_trait]
impl PipelineAssetMutator for MeshVariationDatabaseMutator {
    fn asset_type(&self) -> &'static TypeInfo {
        MESHVARIATIONDATABASE_TYPE_INFO
    }

    fn max_concurrent_jobs(&self) -> usize {
        4
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

        let asset = primary
            .as_any_mut()
            .downcast_mut::<MeshVariationDatabase>()
            .map_or_else(
                || {
                    Err(PipelineMutationError::Custom(
                        "Asset is not a MeshVariationDatabase".to_string(),
                    ))
                },
                |asset| Ok(asset),
            )?;

        for entry in &mut asset.entries {
            let entry = entry
                .as_any_mut()
                .downcast_mut::<MeshVariationDatabaseEntry>()
                .unwrap();

            let asset = match domain
                .load_asset_from_stub_instance(entry.mesh.as_ref().expect("Mesh is None"))
                .await
            {
                Ok(asset) => asset,
                Err(err) => {
                    warn!("Failed to load asset: {:?}", err);
                    continue;
                }
            };

            for material in &mut entry.materials {
                let material = material
                    .as_any_mut()
                    .downcast_mut::<MeshVariationDatabaseMaterial>()
                    .expect("Material is None");

                let material_stub = material.material.as_ref().expect("Material is None");
                let material_id = get_dc_instance_guid(material_stub).await;

                let material_instance = match asset.instance_by_guid(&material_id).await {
                    Some(instance) => instance,
                    None => {
                        warn!(
                            "Material instance {:?} not found in asset {:?}",
                            material_id,
                            asset.guid()
                        );
                        continue;
                    }
                };

                let mut material_instance = material_instance.lock().await;
                let mesh_material = material_instance
                    .as_any_mut()
                    .downcast_mut::<MeshMaterial>()
                    .expect("Material is not a MeshMaterial");

                mem::swap(
                    &mut mesh_material.shader.texture_parameters,
                    &mut material.texture_parameters,
                );
            }

            result.save_asset(asset);
        }

        result.delete_asset(*partition.guid());
        Ok(())
    }
}
