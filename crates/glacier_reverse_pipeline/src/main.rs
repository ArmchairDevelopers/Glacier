use std::{path::PathBuf, sync::Arc};

use glacier_reflect::type_registry::TypeRegistry;
use glacier_reflect_swbf2::register_mod_types;
use glacier_reverse_pipeline::PackagedConversionContext;

use clap::Parser;
use tracing::{error, info, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Convert the target data of a Frostbite game to source data that can be manipulated by Glacier
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to game source data, containing Data and Patch folders
    #[arg(long)]
    source_data: PathBuf,

    /// The path to Glacier output data
    #[arg(long)]
    output_data: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .with(fmt::Layer::default())
        .init();

    if args.source_data.join("Data").exists() == false {
        error!("Source data does not contain a Data folder");
        return;
    }

    info!("Starting up...");

    let ctx = PackagedConversionContext::new(args.source_data, args.output_data);

    let mut registry = TypeRegistry::default();
    register_mod_types(&mut registry);

    glacier_reverse_pipeline::execute(&ctx, Arc::new(registry))
        .await
        .unwrap();
}
