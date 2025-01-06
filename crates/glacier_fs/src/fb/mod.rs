use std::{io::Cursor, path::PathBuf};

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt, SeekFrom},
};

use bundle::BundleManifest;

use crate::{
    fb::{
        catalog::parse_catalog,
        db_object::{read_db_object, DbObject},
        manifest::{parse_manifest, resolve_cas_path},
    },
    io::native_reader::NativeReader,
};

pub mod bundle;
pub mod cas;
pub mod catalog;
pub mod db_object;
pub mod hash;
pub mod manifest;
pub mod memory_fs;

#[derive(thiserror::Error, Debug)]
pub enum FbFileError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    DbObject(#[from] db_object::DbObjectError),
    #[error(transparent)]
    Catalog(#[from] catalog::FbCatalogError),
    #[error("Unknown")]
    Unknown,
    #[error("invalid magic")]
    InvalidMagic,
}

#[derive(Clone, Debug)]
pub struct ConverterContext {
    data_path: PathBuf,
    catalogs: Vec<String>,
}

impl ConverterContext {
    pub fn new(data_path: impl Into<PathBuf>) -> Self {
        Self {
            data_path: data_path.into(),
            catalogs: Vec::new(),
        }
    }
}

pub struct FrostbiteGameData {
    pub ctx: ConverterContext,
    pub bundles: Vec<BundleManifest>,
}

pub async fn read_fb_game_data(
    mut ctx: ConverterContext,
) -> Result<FrostbiteGameData, FbFileError> {
    let mut buffer = Vec::new();

    File::open(ctx.data_path.join("Data/layout.toc"))
        .await?
        .read_to_end(&mut buffer)
        .await?;

    let mut buf = NativeReader::from_bytes(buffer);
    buf.skip(0x22C);
    let object = read_db_object(&mut buf)?.unwrap().1;

    let obj = object.get_as_object()?;

    let catalog = obj.get("installManifest").unwrap();
    let catalog = catalog.get_as_object()?;

    if let DbObject::List(install_chunks) = catalog.get("installChunks").unwrap().get_as_list()? {
        for chunk in install_chunks {
            let chunk = chunk.get_as_object()?;

            let chunk_name = chunk.get("name").unwrap().get_as_str()?.to_owned();
            let chunk_path = "Win32/".to_owned() + &chunk_name;
            let physical_path = ctx.data_path.join("Data").join(&chunk_path);
            if !physical_path.exists() {
                continue;
            }

            parse_catalog(
                ctx.data_path
                    .join("Patch")
                    .join(&chunk_path)
                    .join("cas.cat"),
            )?;

            ctx.catalogs.push(chunk_path);
        }
    }

    let manifest = obj.get("manifest").unwrap();
    let manifest = manifest.get_as_object()?;
    let manifest = parse_manifest(&ctx, manifest).await?;

    let mut bundles = Vec::new();

    for bundle in manifest.bundles {
        let manifest_file = bundle.files.get(0).unwrap();
        // let catalog = ctx
        //     .catalogs
        //     .get(get_catalog_index(file.file) as usize)
        //     .unwrap();
        let path = ctx
            .data_path
            .join(resolve_cas_path(&ctx.catalogs, manifest_file.file));
        if !path.exists() {
            println!("Bundle file '{:?}' is missing!", path);
            continue;
        }

        let mut buffer = vec![0u8; manifest_file.size.try_into().unwrap()];
        let mut file = File::open(path).await?;
        file.seek(SeekFrom::Start(manifest_file.offset.into()))
            .await?;
        file.read_exact(&mut buffer)
            .await
            .expect("Error reading file");
        //let mut buf = ByteBuf::from(Bytes::from(buffer));
        //let data_offset = buf.buf().get_u32() + 4;

        let mut cursor = Cursor::new(buffer);
        let manifest = BundleManifest::load(bundle, &mut cursor).await?;

        bundles.push(manifest);

        // for entry in &manifest.ebx_entries {
        //     if !ebx_entries.insert(entry.name.clone()) {
        //         //println!("Duplicate EBX entry: {}", entry.name);
        //     }
        // }

        // ebx_count += manifest.ebx_entries.len();
    }

    Ok(FrostbiteGameData { ctx, bundles })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_fb_game_data() {
        let ctx = ConverterContext {
            data_path: PathBuf::from("C:\\Program Files\\EA Games\\STAR WARS Battlefront II"),
            catalogs: Vec::new(),
        };

        let data = read_fb_game_data(ctx).await.unwrap();
        data.bundles.iter().for_each(|bundle| {
            println!("{:?}", bundle.res_entries.len());
        });
    }
}
