use std::{ops::Index, slice::SliceIndex};

use glacier_util::{guid::Guid, sha1::Sha1};

pub struct NativeReader {
    buf: Vec<u8>,
    pos: usize,
}

impl NativeReader {
    pub fn new() -> Self {
        Self { buf: Vec::new(), pos: 0 }
    }

    pub fn from_bytes(buf: Vec<u8>) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn seek(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub fn skip(&mut self, len: usize) {
        self.pos += len;
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn remaining(&self) -> usize {
        self.len() - self.pos
    }

    pub fn get_u8(&mut self) -> u8 {
        let value = self.buf[self.pos];
        self.pos += 1;
        value
    }

    pub fn get_i8(&mut self) -> i8 {
        let value = self.buf[self.pos] as i8;
        self.pos += 1;
        value
    }

    pub fn get_u16(&mut self) -> u16 {
        assert!(self.pos + 2 <= self.buf.len());

        let bytes = self.buf.get(self.pos..self.pos + 2).unwrap();
        let value = u16::from_le_bytes(bytes.try_into().expect("slice with incorrect length"));
        self.pos += 2;
        value
    }

    pub fn get_i16(&mut self) -> i16 {
        assert!(self.pos + 2 <= self.buf.len());

        let bytes = self.buf.get(self.pos..self.pos + 2).unwrap();
        let value = i16::from_le_bytes(bytes.try_into().expect("slice with incorrect length"));
        self.pos += 2;
        value
    }

    pub fn get_u32(&mut self) -> u32 {
        assert!(self.pos + 4 <= self.buf.len());

        let bytes = self.buf.get(self.pos..self.pos + 4).unwrap();
        let value = u32::from_le_bytes(bytes.try_into().expect("slice with incorrect length"));
        self.pos += 4;
        value
    }

    pub fn get_i32(&mut self) -> i32 {
        assert!(self.pos + 4 <= self.buf.len());

        let bytes = self.buf.get(self.pos..self.pos + 4).unwrap();
        let value = i32::from_le_bytes(bytes.try_into().expect("slice with incorrect length"));
        self.pos += 4;
        value
    }

    pub fn get_u64(&mut self) -> u64 {
        assert!(self.pos + 8 <= self.buf.len());

        let bytes = self.buf.get(self.pos..self.pos + 8).unwrap();
        let value = u64::from_le_bytes(bytes.try_into().expect("slice with incorrect length"));
        self.pos += 8;
        value
    }

    pub fn get_i64(&mut self) -> i64 {
        assert!(self.pos + 8 <= self.buf.len());

        let bytes = self.buf.get(self.pos..self.pos + 8).unwrap();
        let value = i64::from_le_bytes(bytes.try_into().expect("slice with incorrect length"));
        self.pos += 8;
        value
    }

    pub fn get_f32(&mut self) -> f32 {
        assert!(self.pos + 4 <= self.buf.len());

        let bytes = self.buf.get(self.pos..self.pos + 4).unwrap();
        let value = f32::from_le_bytes(bytes.try_into().expect("slice with incorrect length"));
        self.pos += 4;
        value
    }

    pub fn get_f64(&mut self) -> f64 {
        assert!(self.pos + 8 <= self.buf.len());

        let bytes = self.buf.get(self.pos..self.pos + 8).unwrap();
        let value = f64::from_le_bytes(bytes.try_into().expect("slice with incorrect length"));
        self.pos += 8;
        value
    }

    pub fn get_guid(&mut self) -> Guid {
        Guid::from_slice(&self.get_slice(16))
    }

    pub fn get_sha1(&mut self) -> Sha1 {
        Sha1::from_slice(&self.get_slice(20))
    }

    pub fn get_null_terminated_str(&mut self) -> String {
        let mut bytes = Vec::new();
    
        loop {
            let b = self.get_u8();
            if b == 0 {
                break;
            }
    
            bytes.push(b);
        }
    
        String::from_utf8(bytes).unwrap()
    }
    
    pub fn get_sized_str(&mut self, size: u32) -> String {
        let mut bytes = Vec::new();
    
        for _ in 0..size {
            let b = self.get_u8();
            if b == 0 {
                break;
            }
    
            bytes.push(b);
        }
    
        String::from_utf8(bytes).unwrap()
    }
    
    pub fn get_7bit_encoded_int(&mut self) -> u32 {
        let mut result: u32 = 0;
        let mut i: u32 = 0;
    
        loop {
            let b = self.get_u8() as u32;
            result |= (b & 127) << i;
    
            if b >> 7 == 0 {
                return result;
            }
    
            i += 7;
        }
    }
    
    pub fn get_7bit_encoded_long(&mut self) -> u64 {
        let mut result: u64 = 0;
        let mut i: u32 = 0;
    
        loop {
            let b = self.get_u8() as u32;
            result |= ((b & 127) << i) as u64;
    
            if b >> 7 == 0 {
                return result;
            }
    
            i += 7;
        }
    }

    pub fn get_slice(&mut self, len: usize) -> &[u8] {
        assert!(self.pos + len <= self.buf.len());

        let slice = self.buf.get(self.pos..self.pos + len).unwrap();
        self.pos += len;
        slice
    }

    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.buf.get(index)
    }

    pub fn copy_to_slice(&mut self, slice: &mut [u8]) {
        assert!(self.pos + slice.len() <= self.buf.len());

        slice.copy_from_slice(&self.buf[self.pos..self.pos + slice.len()]);
        self.pos += slice.len();
    }

    pub fn align(&mut self, alignment: usize) {
        self.pos = (self.pos + alignment - 1) & !(alignment - 1);
    }
}

// [..] operator
impl Index<usize> for NativeReader {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl Index<std::ops::Range<usize>> for NativeReader {
    type Output = [u8];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.buf[self.pos + index.start..self.pos + index.end]
    }
}

impl Index<std::ops::RangeFrom<usize>> for NativeReader {
    type Output = [u8];

    fn index(&self, index: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.buf[self.pos..self.pos + index.start]
    }
}

impl Index<std::ops::RangeTo<usize>> for NativeReader {
    type Output = [u8];

    fn index(&self, index: std::ops::RangeTo<usize>) -> &Self::Output {
        &self.buf[self.pos..self.pos + index.end]
    }
}
