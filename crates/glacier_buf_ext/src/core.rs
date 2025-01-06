use std::future::Future;

use tokio::io::{self, AsyncReadExt};

pub trait NullTerminatedStrAsyncReadExt: AsyncReadExt {
    fn read_null_terminated_str(&mut self) -> impl Future<Output = io::Result<String>>
    where
        Self: Unpin,
    {
        async move {
            let mut bytes = Vec::new();

            loop {
                let b = self.read_u8().await?;
                if b == 0 {
                    break;
                }

                bytes.push(b);
            }

            Ok(String::from_utf8(bytes).unwrap())
        }
    }
}

impl<T: AsyncReadExt + Unpin + ?Sized> NullTerminatedStrAsyncReadExt for T {}
