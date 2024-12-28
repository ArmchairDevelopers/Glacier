use std::sync::Arc;

use glacier_fs::fb::{read_fb_game_data, ConverterContext};
use glacier_reflect::type_registry::TypeRegistry;
use packaged_conversion::{
    dbx_conversion::convert_ebx, ebx_indexing::index_ebx, memory_fs::convert_memory_fs,
    PackagedConversionContext,
};

use tracing::info;

pub mod packaged_conversion;

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

    info!("Conversion complete!");
    Ok(())
}
