use std::{hash::{DefaultHasher, Hash, Hasher}, io::SeekFrom};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt};
use tracing::error;

use glacier_buf_ext::core::NullTerminatedStrAsyncReadExt;

use crate::io::native_reader::NativeReader;

use super::{
    manifest::{ManifestBundleInfo, ManifestFileInfo},
    FbFileError,
};

#[derive(Clone, Debug)]
pub struct BundleEbxEntry {
    pub name: String,
    pub original_size: u32,
    pub file: ManifestFileInfo,
}

#[derive(Clone, Debug)]
pub struct BundleResEntry {
    pub name: String,
    pub original_size: u32,
    pub file: ManifestFileInfo,

    pub res_type: u32,
    pub res_meta: [u8; 0x10],
    pub res_id: u64,
}

pub struct BundleManifest {
    pub hash: u32,
    pub name: String,
    pub ebx_entries: Vec<BundleEbxEntry>,
    pub res_entries: Vec<BundleResEntry>,
}

impl BundleManifest {
    pub async fn load<T: AsyncRead + AsyncSeek + Unpin>(
        info: ManifestBundleInfo,
        buf: &mut T,
    ) -> Result<Self, FbFileError> {
        let _data_offset = buf.read_u32().await? + 4;

        let magic = buf.read_u32().await?;
        if magic != 0x9D798ED5 {
            error!("Invalid magic: 0x{:X}", magic);
            return Err(FbFileError::InvalidMagic);
        }

        let total_count = buf.read_u32().await?;
        let ebx_count = buf.read_u32().await?;
        let res_count = buf.read_u32().await?;
        let chunk_count = buf.read_u32().await?;

        assert_eq!(total_count, ebx_count + res_count + chunk_count);

        let strings_offset = buf.read_u32().await? as u64;
        let _meta_offset = buf.read_u32().await?;
        let _data_size = buf.read_u32().await?;

        // Skip Sha1s
        buf.seek(SeekFrom::Current(total_count as i64 * 20)).await?;

        let mut file_index = 1;

        let mut ebx_entries = Vec::with_capacity(ebx_count as usize);
        for _ in 0..ebx_count {
            if file_index >= info.files.len() {
                error!(
                    "Warning: EBX count exceeds file count: {} >= {} >= {}",
                    file_index,
                    info.files.len(),
                    ebx_count
                );
                break;
            }

            let name_offset = buf.read_u32().await? as u64;
            let original_size = buf.read_u32().await?;

            let pos = buf.stream_position().await?;
            buf.seek(SeekFrom::Start(4 + strings_offset + name_offset)).await?;
            let name = buf.read_null_terminated_str().await?;
            buf.seek(SeekFrom::Start(pos)).await?;

            ebx_entries.push(BundleEbxEntry {
                name,
                original_size,
                file: info.files[file_index].clone(),
            });

            file_index += 1;
        }

        let mut res_entries = Vec::with_capacity(res_count as usize);
        for _ in 0..res_count {
            if file_index >= info.files.len() {
                error!(
                    "Warning: RES count exceeds file count: {} >= {} >= {}",
                    file_index,
                    info.files.len(),
                    ebx_count
                );
                break;
            }

            let name_offset = buf.read_u32().await? as u64;
            let original_size = buf.read_u32().await?;

            let pos = buf.stream_position().await?;
            buf.seek(SeekFrom::Start(4 + strings_offset + name_offset)).await?;
            let name = buf.read_null_terminated_str().await?;
            buf.seek(SeekFrom::Start(pos)).await?;

            res_entries.push(BundleResEntry {
                name,
                original_size,
                file: info.files[file_index].clone(),
                res_type: 0,
                res_meta: [0; 0x10],
                res_id: 0,
            });

            file_index += 1;
        }

        for entry in res_entries.iter_mut() {
            entry.res_type = buf.read_u32().await?;
        }

        for entry in res_entries.iter_mut() {
            buf.read_exact(&mut entry.res_meta).await?;
        }

        for entry in res_entries.iter_mut() {
            entry.res_id = buf.read_u64().await?;

            if entry.res_id == 0 {
                let mut hasher = DefaultHasher::new();
                entry.file.hash(&mut hasher);
                entry.res_id = hasher.finish();
            }
        }

        //println!("{:?}", ebx_entries);

        Ok(Self {
            hash: info.hash,
            name: "".to_string(),
            ebx_entries,
            res_entries,
        })
    }
}
