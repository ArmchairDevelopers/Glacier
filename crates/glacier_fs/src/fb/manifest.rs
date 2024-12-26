use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use crate::{
    fb::{db_object::DbObject, hash::hash_string, ConverterContext},
    io::native_reader::NativeReader,
};

use super::FbFileError;

pub fn get_catalog_index(value: u32) -> u32 {
    (value >> 12) - 1
}

pub fn is_in_patch(value: u32) -> bool {
    (value & 0x100) != 0
}

pub fn get_cas_index(value: u32) -> u32 {
    (value & 0xFF) + 1
}

pub fn get_cas_path(catalogs: &Vec<String>, catalog: u32, cas: u32, patch: bool) -> String {
    let mut str = String::new();

    str += if patch { "Patch/" } else { "Data/" };

    str += catalogs.get(catalog as usize).unwrap();
    str += "/cas_";
    str += &format!("{:02}", cas);
    str += ".cas";

    str
}

pub fn resolve_path(catalogs: &Vec<String>, manifest_ref: u32) -> String {
    get_cas_path(
        catalogs,
        get_catalog_index(manifest_ref),
        get_cas_index(manifest_ref),
        is_in_patch(manifest_ref),
    )
}

#[derive(Clone)]
pub struct ManifestFileInfo {
    pub file: u32,
    pub offset: u32,
    pub size: u64,
    pub chunk: bool,
}

pub struct ManifestBundleInfo {
    pub hash: u32,
    pub files: Vec<ManifestFileInfo>,
}

pub struct Manifest {
    pub bundles: Vec<ManifestBundleInfo>,
}

pub fn parse_manifest(
    ctx: &ConverterContext,
    manifest: &DbObject,
) -> Result<Manifest, FbFileError> {
    let file_ref = *manifest.get("file").unwrap().get_as_int()?;
    let offset = *manifest.get("offset").unwrap().get_as_int()?;
    let size = *manifest.get("size").unwrap().get_as_int()?;

    let file = ctx.data_path.join(resolve_path(&ctx.catalogs, file_ref));
    println!("file: {:?}", file);
    println!("offset: {}", offset);
    println!("size: {}", size);

    let mut buffer = vec![0u8; size.try_into().unwrap()];
    let mut file = File::open(file)?;
    file.seek(SeekFrom::Start(offset.into()))?;
    file.read_exact(&mut buffer).expect("Error reading file");
    let mut buf = NativeReader::from_bytes(buffer);

    let file_count = buf.get_u32();
    let bundle_count = buf.get_u32();
    let chunks_count = buf.get_u32();
    println!(
        "Files: {} Bundles: {} Chunks: {}",
        file_count, bundle_count, chunks_count
    );

    let mut manifest_files: Vec<ManifestFileInfo> = Vec::new();
    for _ in 0..file_count {
        manifest_files.push(ManifestFileInfo {
            file: buf.get_u32(),
            offset: buf.get_u32(),
            size: buf.get_u64(),
            chunk: false,
        });
    }

    let mut manifest_bundles: Vec<ManifestBundleInfo> = Vec::new();
    for _ in 0..bundle_count {
        let hash = buf.get_u32();
        let start_index = buf.get_u32();
        let count = buf.get_u32();

        let unk1 = buf.get_u32();
        let unk2 = buf.get_u32();

        let mut files = Vec::new();
        for i in 0..count {
            let file = manifest_files
                .get((start_index + i) as usize)
                .expect("Failed to find manifest file");
            files.push(file.to_owned());
        }

        let file = files.get(0).unwrap();
        let catalog = ctx
            .catalogs
            .get(get_catalog_index(file.file) as usize)
            .unwrap();
        let path = resolve_path(&ctx.catalogs, file.file);
        if hash_string("win32/gameplay/wrgameconfiguration") == hash as u64 {
            println!(
                "File count in {}: {} ({}, {})",
                path,
                count,
                hash,
                hash_string("win32/gameplay/wrgameconfiguration")
            );
            println!("{} {}", files[0].offset, files[0].size);
        }

        manifest_bundles.push(ManifestBundleInfo { hash, files });
    }

    println!("Bundles: {}", manifest_bundles.len());

    for _ in 0..chunks_count {
        let guid = buf.get_guid();
        let file_index = buf.get_i32();
        let manifest_file = manifest_files.get(file_index as usize).unwrap();
        if guid.to_string() == "6a65ef12-952f-8d8c-cddc-98c61e24516b" {
            println!(
                "Manifest file: {}/{}/{}",
                get_catalog_index(manifest_file.file),
                is_in_patch(manifest_file.file),
                get_cas_index(manifest_file.file)
            );
            println!(
                "Manifest file: {}/{}/{}",
                resolve_path(&ctx.catalogs, manifest_file.file),
                manifest_file.offset,
                manifest_file.size
            );
        }
    }

    Ok(Manifest {
        bundles: manifest_bundles,
    })
}
