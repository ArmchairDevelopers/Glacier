use std::sync::Arc;

use glacier_store::{domain::DomainStore, index::asset_index::DomainAssetIndex};
use tracing::{error, info};

use crate::mutator::{
    network_registry::NetworkRegistryMutator, PipelineAssetMutator, UnbuildResult,
};

pub struct ReversePipeline {
    domain: Arc<DomainStore>,
    asset_mutators: Vec<Box<dyn PipelineAssetMutator>>,
}

impl ReversePipeline {
    pub fn new(domain: Arc<DomainStore>) -> Self {
        let mut asset_mutators: Vec<Box<dyn PipelineAssetMutator>> = Vec::new();
        asset_mutators.push(Box::new(NetworkRegistryMutator));

        Self {
            domain,
            asset_mutators,
        }
    }

    pub async fn run_mutators(&self) {
        for mutator in &self.asset_mutators {
            self.run_mutator(&self.domain, mutator).await;
        }
    }

    async fn run_mutator(&self, domain: &DomainStore, mutator: &Box<dyn PipelineAssetMutator>) {
        let asset_type = mutator.asset_type();
        let assets = match domain.index().by_type_hash(asset_type.name_hash) {
            Some(assets) => assets,
            None => return,
        };

        info!(
            "Running reverse pipeline for asset type '{}'",
            asset_type.name
        );

        for asset in assets {
            let mut result = UnbuildResult::new();

            info!("Mutating asset: {}", asset.name);

            let partition = domain
                .load_asset(&asset.name)
                .await
                .expect("Failed to load asset");

            // let instance = partition.primary_instance().unwrap();
            // let mut instance = instance.lock().await;

            // let result = mutator.mutate(&mut *instance, &mut result).await;
            // if let Err(err) = result {
            //     error!("Failed to mutate asset: {}", err);
            // }
        }
    }
}
