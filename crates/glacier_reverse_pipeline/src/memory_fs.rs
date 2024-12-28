use glacier_fs::{fb::{memory_fs::FrostbiteMemoryFs, FbFileError}, io::native_reader::NativeReader};
use tokio::fs;

use super::PackagedConversionContext;

pub async fn convert_memory_fs(ctx: &PackagedConversionContext) -> Result<(), FbFileError> {
    let path = ctx.source_data_path.join("Data/initfs_Win32");
    let buf = fs::read(path).await?;

    let mut buf = NativeReader::from_bytes(buf);
    let fs = FrostbiteMemoryFs::load(&mut buf).await.unwrap();
    
    for (name, data) in fs.files {
        let str = std::str::from_utf8(&data).unwrap();

        // Each level bundle has a generated MemoryFs file that contains the startup configuration.
        // We don't need this, and it just bloats the output.
        if str.starts_with("Core.GameConfigurationName") {
            continue;
        }

        let path = ctx.output_data_path.join(name);
        let directory = path.parent().unwrap();
        fs::create_dir_all(directory).await?;
        fs::write(path, str).await?;
    }

    Ok(())
}
