use async_compression::tokio::bufread::ZstdDecoder;
use bytes::Buf;
use std::io::{self, Cursor};
use tokio::io::AsyncReadExt;

pub const CAS_MAX_CHUNK_SIZE: usize = 0x10000;

#[derive(thiserror::Error, Debug)]
pub enum CasDecompressionError {
    #[error("resource too big")]
    ResourceTooBig,
    #[error("decompression failed: {0}")]
    DecompressionFailed(String),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

pub async fn cas_decompress(
    buf: &[u8],
    needed_size: Option<usize>,
) -> Result<Vec<u8>, CasDecompressionError> {
    if buf.len() < 8 {
        return Err(CasDecompressionError::IoError(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Not enough data for chunk metadata",
        )));
    }

    let mut cursor = Cursor::new(buf);
    let mut writer: Vec<u8> = Vec::new();

    while cursor.remaining() > 0 {
        if cursor.remaining() < 8 {
            return Err(CasDecompressionError::IoError(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Not enough data for chunk metadata",
            )));
        }

        let decompressed_size = cursor.get_u32() as usize;

        let mut compression_type = cursor.get_u16_le();
        let mut buffer_size = cursor.get_u16() as usize;

        let flags = (compression_type & 0xFF00) >> 8;
        if (flags & 0x0F) != 0 {
            buffer_size = ((flags as usize & 0x0F) << 0x10) + buffer_size;
        }

        if buffer_size & 0xFF000000 != 0 {
            buffer_size = buffer_size & 0x00FFFFFF;
        }

        if buffer_size > CAS_MAX_CHUNK_SIZE {
            eprintln!(
                "Failing to read resource of size {} (remaining {})",
                buffer_size,
                cursor.remaining()
            );
            return Err(CasDecompressionError::ResourceTooBig);
        }

        let mut do_decompression = true;

        compression_type = compression_type & 0x7F;

        if compression_type == 0x0 {
            do_decompression = false;
        } else if compression_type != 0x0F {
            return Err(CasDecompressionError::DecompressionFailed(format!(
                "Unsupported compression code: 0x{:X}",
                compression_type
            )));
        }

        let mut buffered_data = vec![0u8; buffer_size];
        cursor.copy_to_slice(&mut buffered_data);

        if do_decompression {
    
            let mut decoder = ZstdDecoder::new(Cursor::new(buffered_data));
            let mut decompressed_data = vec![0u8; decompressed_size];
    
            decoder.read_exact(&mut decompressed_data).await?;
            writer.extend_from_slice(&decompressed_data);
        } else {
            writer.extend_from_slice(&buffered_data);
        }

        if let Some(needed_size) = needed_size {
            if writer.len() >= needed_size {
                //writer.truncate(needed_size);
                break;
            }
        }
    }

    Ok(writer)
}
