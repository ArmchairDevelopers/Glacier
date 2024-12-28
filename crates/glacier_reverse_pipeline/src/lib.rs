use std::{path::PathBuf, sync::Arc};

use crate::{dbx_conversion::convert_ebx, ebx_indexing::index_ebx, memory_fs::convert_memory_fs};
use glacier_fs::fb::{read_fb_game_data, ConverterContext};
use glacier_reflect::type_registry::TypeRegistry;

use tokio::fs;
use tracing::info;

pub mod dbx_conversion;
pub mod ebx_indexing;
pub mod memory_fs;

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
    let conv_ctx = ConverterContext::new(ctx.source_data_path.clone());

    info!("Initializing game data...");

    let data = Arc::new(
        read_fb_game_data(conv_ctx)
            .await
            .expect("Failed to read game data"),
    );

    info!("Indexing EBX...");

    index_ebx(ctx, &registry, &data).await;

    info!("Converting EBX...");

    convert_ebx(ctx, &registry, &data).await;

    info!("Converting memfs...");

    convert_memory_fs(&ctx).await?;

    info!("Pipeline complete!");
    Ok(())
}
