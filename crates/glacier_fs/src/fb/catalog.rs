use std::{fs::File, io::Read, path::PathBuf};

use crate::io::native_reader::NativeReader;
use glacier_util::sha1::Sha1;

const MAGIC: &str = "NyanNyanNyanNyan";

#[derive(thiserror::Error, Debug)]
pub enum FbCatalogError {
    #[error("invalid catalog path '{0}'")]
    InvalidPath(String),
    #[error("invalid catalog header")]
    InvalidHeader,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct CatResourceEntry {
    pub sha1: Sha1,
    pub offset: u32,
    pub size: u32,
    pub logical_offset: u32,
    pub archive_index: u32,
}

impl CatResourceEntry {
    pub fn load(buf: &mut NativeReader) -> Self {
        Self {
            sha1: buf.get_sha1(),
            offset: buf.get_u32(),
            size: buf.get_u32(),
            logical_offset: buf.get_u32(),
            archive_index: buf.get_u32() & 0xFF,
        }
    }
}

pub struct CatPatchEntry {
    pub sha1: Sha1,
    pub base_sha1: Sha1,
    pub delta_sha1: Sha1,
}

impl CatPatchEntry {
    pub fn load(buf: &mut NativeReader) -> Self {
        Self {
            sha1: buf.get_sha1(),
            base_sha1: buf.get_sha1(),
            delta_sha1: buf.get_sha1(),
        }
    }
}

pub struct Catalog {
    pub resource_cnt: u32,
    pub patch_cnt: u32,
    pub encrypted_cnt: u32,
    pub resource_entries: Vec<CatResourceEntry>,
    pub patch_entries: Vec<CatPatchEntry>,
}

pub fn parse_catalog(path: PathBuf) -> Result<Catalog, FbCatalogError> {
    if !path.exists() {
        return Err(FbCatalogError::InvalidPath(
            path.to_string_lossy().to_string(),
        ));
    }

    let mut buffer = Vec::new();
    File::open(&path)?.read_to_end(&mut buffer)?;
    let mut buf = NativeReader::from_bytes(buffer);
    buf.skip(0x22C);

    let magic = buf.get_sized_str(16);
    if magic != MAGIC {
        return Err(FbCatalogError::InvalidHeader);
    }

    let resource_cnt = buf.get_u32();
    let patch_cnt = buf.get_u32();
    let encrypted_cnt = buf.get_u32();
    buf.skip(12);

    println!("{:?} has resources: {}", path, resource_cnt);
    let mut file_count = 0;

    let mut resource_entries = Vec::new();
    for _ in 0..resource_cnt {
        let entry = CatResourceEntry::load(&mut buf);
        file_count = std::cmp::max(entry.archive_index, file_count);
        resource_entries.push(entry);
    }

    let mut patch_entries = Vec::new();
    for _ in 0..patch_cnt {
        let entry = CatPatchEntry::load(&mut buf);
        patch_entries.push(entry);
    }

    println!("File count: {}", file_count);

    Ok(Catalog {
        resource_cnt,
        patch_cnt,
        encrypted_cnt,
        resource_entries,
        patch_entries,
    })
}
