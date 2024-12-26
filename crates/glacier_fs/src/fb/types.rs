use std::{fmt, io::Cursor};

#[derive(Serialize)]
pub struct Sha1 {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
}

impl Sha1 {
    // const ZERO: Sha1 = Sha1 {
    //     a: 0,
    //     b: 0,
    //     c: 0,
    //     d: 0,
    //     e: 0,
    // };

    fn new(bytes: &[u8]) -> Result<Sha1> {
        if bytes.len() < 20 {
            bail!("bytes must be at least 20 bytes long");
        }

        let a = (bytes[0] as u32) | ((bytes[1] as u32) << 8) | ((bytes[2] as u32) << 16) | ((bytes[3] as u32) << 24);
        let b = (bytes[4] as u32) | ((bytes[4 + 1] as u32) << 8) | ((bytes[4 + 2] as u32) << 16) | ((bytes[4 + 3] as u32) << 24);
        let c = (bytes[2 * 4] as u32) | ((bytes[(2 * 4) + 1] as u32) << 8) | ((bytes[(2 * 4) + 2] as u32) << 16) | ((bytes[(2 * 4) + 3] as u32) << 24);
        let d = (bytes[3 * 4] as u32) | ((bytes[(3 * 4) + 1] as u32) << 8) | ((bytes[(3 * 4) + 2] as u32) << 16) | ((bytes[(3 * 4) + 3] as u32) << 24);
        let e = (bytes[4 * 4] as u32) | ((bytes[(4 * 4) + 1] as u32) << 8) | ((bytes[(4 * 4) + 2] as u32) << 16) | ((bytes[(4 * 4) + 3] as u32) << 24);

        Ok(Sha1 { a, b, c, d, e })
    }
}

pub fn read_sha1(buf: &mut Bytes) -> Result<Sha1> {
    let mut bytes = [0u8; 20];
    for i in 0..20 {
        bytes[i] = buf.get_u8();
    }

    Ok(Sha1::new(&bytes)?)
}

impl fmt::Display for Sha1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08x}{:08x}{:08x}{:08x}{:08x}", self.a, self.b, self.c, self.d, self.e)
    }
}

impl fmt::Debug for Sha1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08x}{:08x}{:08x}{:08x}{:08x}", self.a, self.b, self.c, self.d, self.e)
    }
}

use anyhow::{Result, bail};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use bytes::{Bytes, Buf};
use serde::Serialize;
use uuid::Uuid;

pub fn read_guid(buf: &mut Bytes, little_endian: bool) -> Result<Uuid, std::io::Error> {
    let mut bytes = [0u8; 16];
    for i in 0..16 {
        bytes[i] = buf.get_u8();
    }

    let uuid = if little_endian {
        guid_to_uuid(&bytes)?
    } else {
        let mut reversed_bytes = [0u8; 16];
        reversed_bytes[0] = bytes[3];
        reversed_bytes[1] = bytes[2];
        reversed_bytes[2] = bytes[1];
        reversed_bytes[3] = bytes[0];
        reversed_bytes[4] = bytes[5];
        reversed_bytes[5] = bytes[4];
        reversed_bytes[6] = bytes[7];
        reversed_bytes[7] = bytes[6];
        reversed_bytes[8..].copy_from_slice(&bytes[8..]);
        guid_to_uuid(&bytes)?
    };

    Ok(uuid)
}

fn guid_to_uuid(guid: &[u8]) -> Result<Uuid, std::io::Error> {
    let mut source = Cursor::new(guid);
    let mut target = vec![0u8; 0];

    target.write_i32::<LittleEndian>(source.read_i32::<BigEndian>()?)?;
    target.write_i16::<LittleEndian>(source.read_i16::<BigEndian>()?)?;
    target.write_i16::<LittleEndian>(source.read_i16::<BigEndian>()?)?;
    target.write_u64::<BigEndian>(source.read_u64::<BigEndian>()?)?;

    Ok(Uuid::from_slice(&target).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?)
}