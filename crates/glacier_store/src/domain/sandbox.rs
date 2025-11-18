use crate::domain::DomainStoreError;
use async_zip::error::ZipError;
use async_zip::tokio::{read::seek::ZipFileReader, write::ZipFileWriter};
use async_zip::{StoredZipEntry, ZipEntryBuilder};
use futures_lite::io::AsyncReadExt;
use log::error;
use std::io::Cursor;
use std::ops::{Deref, DerefMut};
use std::{collections::HashMap, path::PathBuf};
use tokio::fs;
use tokio::fs::File;
use tokio::io::{AsyncBufRead, AsyncSeek, AsyncWrite};
use tokio::io::{AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::OwnedRwLockWriteGuard;

pub struct Sandbox<T: AsyncBufRead + AsyncSeek + Unpin> {
    reader: ZipFileReader<T>,
    written_files: HashMap<String, Vec<u8>>,
}

impl<T: AsyncBufRead + AsyncSeek + Unpin> Sandbox<T> {
    pub async fn read(buf: T) -> Result<Self, ZipError> {
        let mut reader = ZipFileReader::with_tokio(buf).await?;

        Ok(Self {
            reader,
            written_files: Default::default(),
        })
    }

    pub async fn list_files(&self) -> Vec<String> {
        let mut files = Vec::new();
        for ele in self.reader.file().entries() {
            files.push(ele.filename().as_str().unwrap().to_string());
        }
        files
    }

    pub async fn list_files_in_dir(&self, dir: &str) -> Vec<String> {
        let mut files = Vec::new();
        for ele in self.reader.file().entries() {
            let filename = ele.filename().as_str().unwrap();
            if filename.starts_with(dir) {
                files.push(filename.to_string());
            }
        }
        files
    }

    pub async fn read_file(&mut self, filename: &str) -> Option<Vec<u8>> {
        let mut index: isize = -1;

        let mut i = 0;
        for ele in self.reader.file().entries() {
            if ele.filename().as_str().unwrap() == filename {
                index = i;
                break;
            }

            i += 1;
        }

        if index == -1 {
            return None;
        }

        let mut buf = Vec::new();
        let mut reader = self
            .reader
            .reader_without_entry(index as usize)
            .await
            .expect("Failed to open file");
        reader
            .read_to_end(&mut buf)
            .await
            .expect("Failed to read file");
        Some(buf)
    }

    pub async fn read_file_as_string(&mut self, filename: &str) -> Option<String> {
        self.read_file(filename)
            .await
            .and_then(|x| Some(String::from_utf8(x).unwrap()))
    }

    pub fn write_file(&mut self, filename: &str, buf: Vec<u8>) {
        self.written_files.insert(filename.to_owned(), buf);
    }

    pub async fn commit<W: AsyncWrite + AsyncSeek + Unpin>(mut self, buf: &mut W) {
        let mut writer = ZipFileWriter::with_tokio(buf);

        let mut index = 0;
        for ele in self
            .reader
            .file()
            .entries()
            .iter()
            .cloned()
            .collect::<Vec<StoredZipEntry>>()
        {
            let filename = ele.filename().as_str().unwrap();
            if self.written_files.contains_key(filename) {
                index += 1;
                continue;
            }

            let mut buf = Vec::new();
            let mut reader = self
                .reader
                .reader_without_entry(index)
                .await
                .expect("TODO: panic message");
            reader
                .read_to_end(&mut buf)
                .await
                .expect("TODO: panic message");

            let entry =
                ZipEntryBuilder::new(filename.into(), async_zip::Compression::Stored).build();
            writer
                .write_entry_whole(entry, &buf)
                .await
                .expect("TODO: panic message");

            index += 1;
        }

        for (filename, buf) in self.written_files {
            let entry =
                ZipEntryBuilder::new(filename.into(), async_zip::Compression::Stored).build();
            writer
                .write_entry_whole(entry, &buf)
                .await
                .expect("TODO: panic message");
        }

        writer.close().await.expect("TODO: panic message");
    }
}

async fn create_empty_zip() -> Vec<u8> {
    let mut writer = Cursor::new(Vec::new());
    let mut zip_writer = ZipFileWriter::with_tokio(&mut writer);
    zip_writer.close().await.expect("TODO: panic message");
    writer.flush().await.expect("TODO: panic message");
    writer.into_inner()
}

pub struct DomainSandbox {
    lock: OwnedRwLockWriteGuard<()>,

    path: PathBuf,
    sandbox: Sandbox<BufReader<Cursor<Vec<u8>>>>,
}

impl DomainSandbox {
    pub async fn open(
        lock: OwnedRwLockWriteGuard<()>,
        path: impl Into<PathBuf>,
        write: bool,
    ) -> Result<Self, DomainStoreError> {
        let path = path.into();
        let reader = if fs::try_exists(&path).await? {
            let file = fs::read(&path).await?;
            BufReader::new(Cursor::new(file))
        } else {
            let zip = create_empty_zip().await;
            BufReader::new(Cursor::new(zip))
        };

        let sandbox = match Sandbox::read(reader).await {
            Ok(sandbox) => sandbox,
            Err(err) => {
                error!("Failed to open sandbox: {}", err);
                let zip = create_empty_zip().await;
                Sandbox::read(BufReader::new(Cursor::new(zip))).await?
            }
        };

        Ok(Self {
            lock,
            path,
            sandbox,
        })
    }

    pub async fn commit(self) {
        let file = File::create(&self.path).await.expect("TODO: panic message");
        let mut writer = BufWriter::new(file);
        self.sandbox.commit(&mut writer).await;
        writer.flush().await.expect("TODO: panic message");
    }
}

impl Deref for DomainSandbox {
    type Target = Sandbox<BufReader<Cursor<Vec<u8>>>>;

    fn deref(&self) -> &Self::Target {
        &self.sandbox
    }
}

impl DerefMut for DomainSandbox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sandbox
    }
}
