use std::future::Future;

use glacier_util::guid::Guid;
use tokio::io::{self, AsyncReadExt};

pub trait GuidAsyncReadExt: AsyncReadExt {
    fn read_guid(&mut self) -> impl Future<Output = io::Result<Guid>>
    where
        Self: Unpin,
    {
        async move {
            let mut data = [0u8; 16];
            self.read_exact(&mut data).await?;
            Ok(Guid::from_slice(&data))
        }
    }
}

impl<T: AsyncReadExt + Unpin + ?Sized> GuidAsyncReadExt for T {}
