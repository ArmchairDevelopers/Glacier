use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use bytes::Bytes;

use crate::{
    fb::{
        catalog::parse_catalog,
        db_object::{read_db_object, DbObject},
        manifest::{parse_manifest, resolve_path},
    },
    io::native_reader::NativeReader,
};

pub mod catalog;
pub mod db_object;
pub mod hash;
pub mod manifest;

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
}

pub struct ConverterContext {
    data_path: PathBuf,
    catalogs: Vec<String>,
}

fn read_fb_game_data() -> Result<(), FbFileError> {
    println!("Hello, world!");

    let mut ctx = ConverterContext {
        data_path: PathBuf::from("C:\\Program Files\\EA Games\\STAR WARS Battlefront II"),
        catalogs: Vec::new(),
    };

    let mut buffer = Vec::new();
    File::open(ctx.data_path.join("Data/layout.toc"))?.read_to_end(&mut buffer)?;
    let mut buf = NativeReader::from_bytes(buffer);
    buf.skip(0x22C);
    let object = read_db_object(&mut buf)?.unwrap().1;

    //let data = serde_json::to_string_pretty(&object)?;
    //std::fs::write("test.json", data)?;

    let obj = object.get_as_object()?;

    let catalog = obj.get("installManifest").unwrap();
    let catalog = catalog.get_as_object()?;

    if let Ok(DbObject::List(install_chunks)) = catalog.get("installChunks").unwrap().get_as_list()
    {
        for chunk in install_chunks {
            let chunk = chunk.get_as_object()?;

            let chunk_name = chunk.get("name").unwrap().get_as_str()?.to_owned();
            let chunk_path = "Win32/".to_owned() + &chunk_name;
            let physical_path = ctx.data_path.join("Data").join(&chunk_path);
            //println!("Path name: {}", physical_path.to_str().unwrap());
            if !physical_path.exists() {
                continue;
            }

            //parse_catalog(&ctx, physical_path.join("cas.cat"))?;
            parse_catalog(
                ctx.data_path
                    .join("Patch")
                    .join(&chunk_path)
                    .join("cas.cat"),
            )?;

            ctx.catalogs.push(chunk_path);
        }
    }

    println!("{:?}", ctx.catalogs);

    let manifest = obj.get("manifest").unwrap();
    let manifest = manifest.get_as_object()?;
    let manifest = parse_manifest(&ctx, manifest)?;

    for bundle in manifest.bundles {
        let manifest_file = bundle.files.get(0).unwrap();
        // let catalog = ctx
        //     .catalogs
        //     .get(get_catalog_index(file.file) as usize)
        //     .unwrap();
        let path = ctx
            .data_path
            .join(resolve_path(&ctx.catalogs, manifest_file.file));
        if !path.exists() {
            println!("Bundle file '{:?}' is missing!", path);
            continue;
        }

        let mut buffer = vec![0u8; manifest_file.size.try_into().unwrap()];
        let mut file = File::open(path)?;
        file.seek(SeekFrom::Start(manifest_file.offset.into()))?;
        file.read_exact(&mut buffer).expect("Error reading file");
        //let mut buf = ByteBuf::from(Bytes::from(buffer));
        //let data_offset = buf.buf().get_u32() + 4;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_fb_game_data() {
        assert!(read_fb_game_data().is_ok());
    }
}
