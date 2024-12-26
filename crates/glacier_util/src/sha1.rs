use std::fmt::Debug;

use bytes::BytesMut;

pub struct Sha1 {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
}

impl Sha1 {
    pub fn from_slice(bytes: &[u8]) -> Sha1 {
        assert!(bytes.len() == 20, "slice must be 20 bytes long (length: {})", bytes.len());

        let a = (bytes[0] as u32) | ((bytes[1] as u32) << 8) | ((bytes[2] as u32) << 16) | ((bytes[3] as u32) << 24);
        let b = (bytes[4] as u32) | ((bytes[4 + 1] as u32) << 8) | ((bytes[4 + 2] as u32) << 16) | ((bytes[4 + 3] as u32) << 24);
        let c = (bytes[2 * 4] as u32) | ((bytes[(2 * 4) + 1] as u32) << 8) | ((bytes[(2 * 4) + 2] as u32) << 16) | ((bytes[(2 * 4) + 3] as u32) << 24);
        let d = (bytes[3 * 4] as u32) | ((bytes[(3 * 4) + 1] as u32) << 8) | ((bytes[(3 * 4) + 2] as u32) << 16) | ((bytes[(3 * 4) + 3] as u32) << 24);
        let e = (bytes[4 * 4] as u32) | ((bytes[(4 * 4) + 1] as u32) << 8) | ((bytes[(4 * 4) + 2] as u32) << 16) | ((bytes[(4 * 4) + 3] as u32) << 24);

        Sha1 { a, b, c, d, e }
    }
}

impl Debug for Sha1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08x}{:08x}{:08x}{:08x}{:08x}", self.a, self.b, self.c, self.d, self.e)
    }
}

pub trait BytesSha1Ext {
    fn get_sha1(&mut self) -> Sha1;
}

impl BytesSha1Ext for BytesMut {
    fn get_sha1(&mut self) -> Sha1 {
        Sha1::from_slice(&self.split_to(20))
    }
}

