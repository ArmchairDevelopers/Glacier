use std::path::PathBuf;
use tokio::fs;

/// This module performs the initial conversion from packaged data formats to dbx and sandboxes.

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
        fs::create_dir_all(&path).await.expect("Failed to create state data directory");
        path
    }
}
