use bytes::{Buf, Bytes};

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

pub struct BundleManifest {
    pub hash: u32,
    pub name: String,
    pub ebx_entries: Vec<BundleEbxEntry>,
}

impl BundleManifest {
    pub fn load(info: ManifestBundleInfo, bytes: &mut NativeReader) -> Result<Self, FbFileError> {
        let data_offset = bytes.get_u32_be() + 4;

        let magic = bytes.get_u32_be();
        if magic != 0x9D798ED5 {
            println!("Invalid magic: 0x{:X}", magic);
            return Err(FbFileError::InvalidMagic);
        }

        let total_count = bytes.get_u32_be();
        let ebx_count = bytes.get_u32_be();
        let res_count = bytes.get_u32_be();
        let chunk_count = bytes.get_u32_be();

        assert_eq!(total_count, ebx_count + res_count + chunk_count);

        let strings_offset = bytes.get_u32_be();
        let meta_offset = bytes.get_u32_be();
        let data_size = bytes.get_u32_be();

        // Skip Sha1s
        bytes.skip(total_count as usize * 20);

        let mut file_index = 1;

        let mut ebx_entries = Vec::with_capacity(ebx_count as usize);
        for _ in 0..ebx_count {
            if file_index >= info.files.len() {
                eprintln!("Warning: EBX count exceeds file count: {} >= {} >= {}", file_index, info.files.len(), ebx_count);
                break;
            }

            let name_offset = bytes.get_u32_be();
            let original_size = bytes.get_u32_be();

            let pos = bytes.pos();
            bytes.seek(4 + strings_offset as usize + name_offset as usize);
            let name = bytes.get_null_terminated_str();
            bytes.seek(pos);

            ebx_entries.push(BundleEbxEntry {
                name,
                original_size,
                file: info.files[file_index].clone(),
            });

            file_index += 1;
        }

        //println!("{:?}", ebx_entries);

        Ok(Self {
            hash: info.hash,
            name: "".to_string(),
            ebx_entries,
        })
    }
}
