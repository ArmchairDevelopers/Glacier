use std::io::Cursor;

use async_compression::tokio::bufread::ZstdDecoder;
use bytes::Buf;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt, SeekFrom},
};

use crate::{
    fb::{db_object::DbObject, ConverterContext},
    io::native_reader::NativeReader,
};

use super::{
    cas::{cas_decompress, CasDecompressionError},
    FbFileError,
};

fn get_catalog_index(value: u32) -> u32 {
    (value >> 12) - 1
}

fn is_in_patch(value: u32) -> bool {
    (value & 0x100) != 0
}

fn get_cas_index(value: u32) -> u32 {
    (value & 0xFF) + 1
}

fn get_cas_path(catalogs: &Vec<String>, catalog: u32, cas: u32, patch: bool) -> String {
    let mut str = String::new();

    str += if patch { "Patch/" } else { "Data/" };

    str += catalogs.get(catalog as usize).unwrap();
    str += "/cas_";
    str += &format!("{:02}", cas);
    str += ".cas";

    str
}

pub fn resolve_cas_path(catalogs: &Vec<String>, manifest_ref: u32) -> String {
    get_cas_path(
        catalogs,
        get_catalog_index(manifest_ref),
        get_cas_index(manifest_ref),
        is_in_patch(manifest_ref),
    )
}

#[derive(Clone, Debug)]
pub struct ManifestFileInfo {
    pub file: u32,
    pub offset: u32,
    pub size: u64,
    pub chunk: bool,
}

impl ManifestFileInfo {
    pub async fn read_data(
        &self,
        ctx: &ConverterContext,
        mut needed_size: Option<usize>,
    ) -> Result<Vec<u8>, CasDecompressionError> {
        let file = ctx
            .data_path
            .join(resolve_cas_path(&ctx.catalogs, self.file));

        if let Some(size) = needed_size {
            if size > self.size as usize {
                needed_size = Some(self.size as usize);
            }
        }

        let mut buffer = vec![0u8; needed_size.unwrap_or(self.size as usize)];

        {
            let mut file = File::open(file).await?;
            file.seek(SeekFrom::Start(self.offset.into())).await?;
            file.read_exact(&mut buffer).await?;
        }

        cas_decompress(&buffer, needed_size).await
    }
}

pub struct ManifestBundleInfo {
    pub hash: u32,
    pub files: Vec<ManifestFileInfo>,
}

pub struct Manifest {
    pub bundles: Vec<ManifestBundleInfo>,
}

pub async fn parse_manifest(
    ctx: &ConverterContext,
    manifest: &DbObject,
) -> Result<Manifest, FbFileError> {
    let mut vba = {
        let vba = include_bytes!("VanillaBundleAggregation.kb");

        let mut cursor = Cursor::new(vba);
        let original_size = cursor.get_i32() as usize;

        let mut decoder = ZstdDecoder::new(cursor);
        let mut decompressed_vba = vec![0u8; original_size];

        decoder
            .read_exact(&mut decompressed_vba)
            .await
            .expect("Failed to decompress VBA");

        let mut cursor = NativeReader::from_bytes(decompressed_vba);
        let magic = cursor.get_i32_be();
        if magic != 0x77778888 {
            return Err(FbFileError::InvalidMagic);
        }

        let catalog_count = cursor.get_i32_be();
        for _ in 0..catalog_count {
            let _ = cursor.get_null_terminated_str();
        }

        cursor
    };

    let file_ref = *manifest.get("file").unwrap().get_as_int()?;
    let offset = *manifest.get("offset").unwrap().get_as_int()?;
    let size = *manifest.get("size").unwrap().get_as_int()?;

    let file = ctx
        .data_path
        .join(resolve_cas_path(&ctx.catalogs, file_ref));

    let mut buffer = vec![0u8; size.try_into().unwrap()];
    let mut file = File::open(file).await?;
    file.seek(SeekFrom::Start(offset.into())).await?;
    file.read_exact(&mut buffer)
        .await
        .expect("Error reading file");
    let mut buf = NativeReader::from_bytes(buffer);

    let file_count = buf.get_u32();
    let bundle_count = buf.get_u32();
    let chunks_count = buf.get_u32();

    let vba_bundle_count = vba.get_u32_be();
    assert_eq!(bundle_count, vba_bundle_count);

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

        let _unk1 = buf.get_u32();
        let _unk2 = buf.get_u32();

        let vba_hash = vba.get_i32_be() as u32;
        assert_eq!(hash, vba_hash);

        let _ebx_count = vba.get_i32_be();
        let _res_count = vba.get_i32_be();
        let _chunk_count = vba.get_i32_be();

        let file_count = vba.get_i32_be() as u32;

        // let mut files = Vec::new();
        // for i in 0..count {
        //     let file = manifest_files
        //         .get((start_index + i) as usize)
        //         .expect("Failed to find manifest file");
        //     files.push(file.to_owned());
        // }

        let mut files = Vec::new();
        // let file = manifest_files
        //     .get(start_index as usize)
        //     .expect("Failed to find manifest file");
        // files.push(file.to_owned());

        for _ in 0..file_count {
            let file = vba.get_i64_be();
            let offset = vba.get_i64_be();
            let size = vba.get_i64_be();

            files.push(ManifestFileInfo {
                file: file as u32,
                offset: offset as u32,
                size: size as u64,
                chunk: false,
            });
        }

        //let file = files.get(0).unwrap();
        // let catalog = ctx
        //     .catalogs
        //     .get(get_catalog_index(file.file) as usize)
        //     .unwrap();
        // //println!("Catalog: {}", catalog);

        // let path = resolve_cas_path(&ctx.catalogs, file.file);
        manifest_bundles.push(ManifestBundleInfo { hash, files });
    }

    for _ in 0..chunks_count {
        let guid = buf.get_guid();
        let file_index = buf.get_i32();
        //let manifest_file = manifest_files.get(file_index as usize).unwrap();
        // if guid.to_string() == "6a65ef12-952f-8d8c-cddc-98c61e24516b" {
        //     println!(
        //         "Manifest file: {}/{}/{}",
        //         get_catalog_index(manifest_file.file),
        //         is_in_patch(manifest_file.file),
        //         get_cas_index(manifest_file.file)
        //     );
        //     println!(
        //         "Manifest file: {}/{}/{}",
        //         resolve_cas_path(&ctx.catalogs, manifest_file.file),
        //         manifest_file.offset,
        //         manifest_file.size
        //     );
        // }
    }

    Ok(Manifest {
        bundles: manifest_bundles,
    })
}
