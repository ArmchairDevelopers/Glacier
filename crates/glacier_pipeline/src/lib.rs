use std::{path::PathBuf, sync::Arc};

use crate::{dbx_conversion::convert_ebx, ebx_indexing::index_ebx, memory_fs::convert_memory_fs};
use bundle_breaker::BundleBreaker;
use glacier_fs::fb::{read_fb_game_data, ConverterContext};
use glacier_reflect::type_registry::TypeRegistry;

use glacier_store::{domain::DomainStore, index::asset_index::DomainAssetIndex};
use pipeline::DataPipeline;
use tokio::{fs, sync::RwLock};
use tracing::info;

pub mod bundle_breaker;
pub mod dbx_conversion;
pub mod ebx_indexing;
pub mod memory_fs;
pub mod mutator;
pub mod pipeline;
pub mod policy;

pub struct PackagedConversionContext {
    pub source_data_path: PathBuf,
    pub output_data_path: PathBuf,
}

impl PackagedConversionContext {
    pub fn new(source_data_path: impl Into<PathBuf>, output_data_path: impl Into<PathBuf>) -> Self {
        Self {
            source_data_path: source_data_path.into(),
            output_data_path: output_data_path.into(),
        }
    }

    pub async fn state_data_path(&self) -> PathBuf {
        let path = self.output_data_path.join(".state");
        fs::create_dir_all(&path)
            .await
            .expect("Failed to create state data directory");
        path
    }

    pub async fn output_domain_path(&self) -> PathBuf {
        let path = self.output_data_path.join("Source");
        fs::create_dir_all(&path)
            .await
            .expect("Failed to create output domain directory");
        path
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ReversePipelineError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Fb(#[from] glacier_fs::fb::FbFileError),
}

pub async fn execute(
    ctx: &PackagedConversionContext,
    registry: Arc<TypeRegistry>,
) -> Result<(), ReversePipelineError> {
    info!("Loading indexed partitions...");

    let index_data = fs::read(ctx.state_data_path().await.join("partition_index"))
        .await
        .expect("Failed to write indexed partitions");
    let asset_index = Arc::new(RwLock::new(
        DomainAssetIndex::load(&index_data).unwrap(),
    ));

    info!("Initializing pipeline...");

    let domain = Arc::new(DomainStore::new(
        registry,
        asset_index,
        ctx.output_data_path.clone(),
        "Source",
    ));

    let pipeline = DataPipeline::new(domain.clone(), data.clone(), false);
    pipeline.run_mutators().await;

    info!("Initializing bundle breaker...");

    let bundle_breaker = BundleBreaker::new();
    bundle_breaker.execute(&domain, &data).await;

    info!("Converting memfs...");

    convert_memory_fs(&ctx).await?;

    info!("Pipeline complete!");
    Ok(())
}
