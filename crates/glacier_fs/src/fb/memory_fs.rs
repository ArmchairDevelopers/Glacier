use std::{collections::HashMap, rc::Rc};

use crate::io::native_reader::NativeReader;

use super::{
    db_object::{read_db_object, DbObject, DbType},
    FbFileError,
};

#[derive(Debug)]
pub struct FrostbiteMemoryFs {
    pub files: HashMap<String, Vec<u8>>,
}

impl FrostbiteMemoryFs {
    pub async fn load(buf: &mut NativeReader) -> Result<Self, FbFileError> {
        buf.skip(0x22C);

        let db_object = match read_db_object(buf)? {
            Some((_, db_object)) => db_object,
            None => return Err(FbFileError::Unknown),
        };

        let mut files = HashMap::new();
        files.extend(Self::load_initfs(db_object)?);

        if let Some(internal) = files.remove("__fsinternal__") {
            let db_object = read_db_object(&mut NativeReader::from_bytes(internal.clone()))?
                .unwrap()
                .1;

            files.extend(Self::load_initfs(db_object)?);
        }

        Ok(Self { files })
    }

    fn load_initfs(db_object: Rc<DbType>) -> Result<HashMap<String, Vec<u8>>, FbFileError> {
        let mut files = HashMap::new();

        if let DbObject::List(list) = db_object.get_as_list()? {
            for entry in list {
                let stub = entry.get_as_object()?.get("$file").unwrap();
                let stub = stub.get_as_object()?;
                let name = stub.get("name").unwrap().get_as_str()?.to_owned();
                let data = stub.get("payload").unwrap().get_as_byte_array()?.clone();
                files.insert(name.to_owned(), data);
            }
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_fb_game_data() {
        let buf = tokio::fs::read(
            r"C:\Program Files\EA Games\STAR WARS Battlefront II\Data\initfs_Win32",
        )
        .await
        .unwrap();

        let mut buf = NativeReader::from_bytes(buf);
        let fs = FrostbiteMemoryFs::load(&mut buf).await.unwrap();
        dbg!(fs.files.keys());
    }
}
