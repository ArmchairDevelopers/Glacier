use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use glacier_fs::{
    db::partition::DatabasePartition,
    dbx::reader::DbxReaderError,
    fb::{bundle::BundleResEntry, FrostbiteGameData},
};
use glacier_store::{domain::DomainStore, index::asset_index::DomainAssetIndex};
use glacier_util::hash::QuickHashExt;
use tokio::sync::{RwLock, Semaphore};
use tracing::{error, info};
use typemap_rev::TypeMap;

use crate::mutator::{
    blueprint::BlueprintMutator, class_info_asset::ClassInfoAssetMutator,
    level_description::LevelDescriptionMutator, mesh_variation_db::MeshVariationDatabaseMutator,
    network_registry::NetworkRegistryMutator, rvm_database::Dx11RvmDatabaseResourceMutator,
    PipelineAssetMutator, PipelineResourceMutator, UnbuildAction, UnbuildResult,
};

struct ResourceIndex {
    rid_to_data: HashMap<u64, BundleResEntry>,
    type_to_rid: HashMap<u32, HashSet<u64>>,
}

impl ResourceIndex {
    pub fn new(data: &FrostbiteGameData) -> Self {
        let mut rid_to_data = HashMap::new();
        let mut type_to_rid: HashMap<u32, HashSet<u64>> = HashMap::new();

        for bundle in &data.bundles {
            for entry in &bundle.res_entries {
                let rid = entry.res_id;
                let type_hash = entry.res_type;

                rid_to_data.insert(rid, entry.clone());
                type_to_rid.entry(type_hash).or_default().insert(rid);
            }
        }

        Self {
            rid_to_data,
            type_to_rid,
        }
    }

    pub fn get(&self, rid: u64) -> Option<&BundleResEntry> {
        self.rid_to_data.get(&rid)
    }

    pub fn get_type(&self, type_hash: u32) -> Option<&HashSet<u64>> {
        self.type_to_rid.get(&type_hash)
    }
}

pub type PipelineStorage = Arc<RwLock<TypeMap>>;

pub struct ReversePipeline {
    domain: Arc<DomainStore>,

    game_data: Arc<FrostbiteGameData>,
    resource_index: ResourceIndex,

    asset_mutators: Vec<Arc<dyn PipelineAssetMutator>>,
    resource_mutators: Vec<Arc<dyn PipelineResourceMutator>>,

    storage: PipelineStorage,

    dry_run: bool,
}

impl ReversePipeline {
    pub fn new(domain: Arc<DomainStore>, game_data: Arc<FrostbiteGameData>, dry_run: bool) -> Self {
        let mut asset_mutators: Vec<Arc<dyn PipelineAssetMutator>> = Vec::new();
        //asset_mutators.push(Arc::new(BlueprintMutator));
        //asset_mutators.push(Arc::new(ClassInfoAssetMutator));
        asset_mutators.push(Arc::new(LevelDescriptionMutator));
        asset_mutators.push(Arc::new(MeshVariationDatabaseMutator));
        asset_mutators.push(Arc::new(NetworkRegistryMutator));

        let mut resource_mutators: Vec<Arc<dyn PipelineResourceMutator>> = Vec::new();
        resource_mutators.push(Arc::new(Dx11RvmDatabaseResourceMutator));

        let resource_index = ResourceIndex::new(&game_data);

        Self {
            domain,
            game_data,
            resource_index,
            asset_mutators,
            resource_mutators,
            storage: Arc::new(RwLock::new(TypeMap::new())),
            dry_run,
        }
    }

    pub async fn run_mutators(&self) {
        for mutator in &self.asset_mutators {
            //self.run_asset_mutator(mutator).await;
        }

        for mutator in &self.resource_mutators {
            self.run_resource_mutator(mutator).await;
        }

        // Save the asset index after all mutators have run
        if let Err(err) = self.domain.save_asset_index().await {
            error!("Failed to save asset index: {}", err);
        }
    }

    async fn run_asset_mutator(&self, mutator: &Arc<dyn PipelineAssetMutator>) {
        let asset_type = mutator.asset_type();

        info!(
            "Running reverse pipeline for asset type '{}'",
            asset_type.name
        );

        let mut processed = 0;

        let max_concurrent_jobs = mutator.max_concurrent_jobs();
        let semaphore = Arc::new(Semaphore::new(max_concurrent_jobs));
        let mut handles = Vec::new();

        let values = self.domain.index_read().await.values_cloned();
        let mut applicable_values = Vec::new();

        for asset in values {
            let entry_type = match self
                .domain
                .type_registry()
                .type_by_hash(asset.primary_type_hash)
            {
                Some(entry_type) => entry_type,
                None => {
                    error!(
                        "Failed to find type for entry with hash {}",
                        asset.primary_type_hash
                    );
                    continue;
                }
            };

            let applicable = asset_type == entry_type
                || self
                    .domain
                    .type_registry()
                    .is_type_descendant(asset_type, entry_type);
            if !applicable {
                continue;
            }

            applicable_values.push(asset);
        }

        let count = applicable_values.len();
        for asset in applicable_values {
            if processed % (count / 10) == 0 {
                info!("Progress: {}%", processed as f32 / count as f32 * 100.0);
            }

            let permit = semaphore.clone().acquire_owned().await.unwrap();

            let domain = self.domain.clone();
            let mutator = mutator.clone();
            let dry_run = self.dry_run;
            handles.push(tokio::spawn(async move {
                let mut unbuild_result = UnbuildResult::new();

                let mut partition = match domain.load_asset(&asset.name).await {
                    Ok(partition) => partition,
                    Err(err) => {
                        if let DbxReaderError::FileNotFound = err {
                            //return;
                        }

                        error!("Failed to load asset '{}': {}", asset.name, err);
                        return;
                    }
                };

                let result = mutator
                    .mutate(&domain, &mut partition, &mut unbuild_result)
                    .await;
                if let Err(err) = result {
                    error!("Failed to mutate asset '{}': {}", asset.name, err);
                }

                drop(mutator);

                if !dry_run {
                    Self::execute_actions(&domain, &partition, unbuild_result).await;
                }

                drop(permit);
            }));

            processed += 1;
        }

        futures::future::join_all(handles).await;

        info!(
            "Processed {} assets of type '{}'",
            processed, asset_type.name
        );
    }

    async fn run_resource_mutator(&self, mutator: &Arc<dyn PipelineResourceMutator>) {
        let resource_type_name = mutator.resource_type_name();

        info!(
            "Running reverse pipeline for resource type '{}'",
            resource_type_name
        );

        {
            let mut unbuild_result = UnbuildResult::new();
            let result = mutator.pre_mutation(&self.domain, &self.storage, &mut unbuild_result).await;
            if let Err(err) = result {
                error!("Failed to run pre-mutation for resource type '{}': {}", resource_type_name, err);
            }
        }

        let mut processed = 0;

        let max_concurrent_jobs = mutator.max_concurrent_jobs();
        let semaphore = Arc::new(Semaphore::new(max_concurrent_jobs));
        let mut handles = Vec::new();

        let values = self
            .resource_index
            .get_type(resource_type_name.to_lowercase().hash_quick())
            .expect("Failed to find resources for type");

        let count = values.len();
        for resource in values {
            let resource = self
                .resource_index
                .get(*resource)
                .expect("Failed to find resource")
                .clone();

            println!("Running reverse pipeline for resource '{}'", resource.name);

            // if processed % (count / 10) == 0 {
            //     info!("Progress: {}%", processed as f32 / count as f32 * 100.0);
            // }

            let permit = semaphore.clone().acquire_owned().await.unwrap();

            let domain = self.domain.clone();
            let ctx = self.game_data.ctx.clone();
            let mutator = mutator.clone();
            let dry_run = self.dry_run;
            let storage = self.storage.clone();
            handles.push(tokio::spawn(async move {
                let mut unbuild_result = UnbuildResult::new();

                let data = match resource.file.read_data(&ctx, None).await {
                    Ok(data) => data,
                    Err(err) => {
                        error!("Failed to read resource '{}': {}", resource.name, err);
                        return;
                    }
                };

                let result = mutator
                    .mutate(
                        &domain,
                        storage,
                        &resource.name,
                        resource.res_id,
                        &resource.res_meta,
                        &data,
                        &mut unbuild_result,
                    )
                    .await;
                if let Err(err) = result {
                    error!("Failed to mutate resource '{}': {}", resource.name, err);
                }

                drop(mutator);

                if !dry_run {
                    // TODO: Execute actions
                }

                drop(permit);
            }));

            processed += 1;
        }

        futures::future::join_all(handles).await;

        {
            let mut unbuild_result = UnbuildResult::new();
            let result = mutator.post_mutation(&self.domain, &self.storage, &mut unbuild_result).await;
            if let Err(err) = result {
                error!("Failed to run post-mutation for resource type '{}': {}", resource_type_name, err);
            }
        }

        info!(
            "Processed {} resources of type '{}'",
            processed, resource_type_name
        );
    }

    async fn execute_actions(
        domain: &DomainStore,
        partition: &DatabasePartition,
        unbuild_result: UnbuildResult,
    ) {
        for action in unbuild_result.actions {
            match action {
                UnbuildAction::SaveSelf => {
                    if let Err(err) = domain.save_asset(partition).await {
                        error!("Failed to save asset: {}", err);
                    }
                }
                UnbuildAction::SaveAsset(asset) => {
                    if let Err(err) = domain.save_asset(&asset).await {
                        error!("Failed to save asset '{}': {}", asset.name(), err);
                    }
                }
                UnbuildAction::DeleteAsset(guid) => {
                    let asset = match domain.index_read().await.get_name_by_partition_guid(&guid) {
                        Some(asset) => asset.to_owned(),
                        None => {
                            error!("Failed to find asset with partition guid {:?}", guid);
                            continue;
                        }
                    };

                    //println!("Deleting asset with partition guid {:?}", guid);
                    if let Err(err) = domain.delete_asset(&asset).await {
                        error!("Failed to delete asset '{}': {}", asset, err);
                    }

                    //info!("Deleted asset '{}'", asset.name);
                }
            }
        }
    }
}
