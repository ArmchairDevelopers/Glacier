use std::num::Wrapping;

// pub fn hash_with_seed(data: &[u8], hash: u64) -> u64 {
//     let mut hash = Wrapping(hash);
//     for &datum in data {
//         hash = hash * Wrapping(33) ^ Wrapping(datum as u64);
//     }
//     hash.0
// }

// pub fn hash_quick_str(str: &str) -> u32 {
//     let l = hash_with_seed(str.to_lowercase().as_bytes(), 5381);
//     (l & 0xFFFFFFFF) as u32
// }

const fn hash_quick_str_const_rec(bytes: &[u8], len: usize, index: usize, hash: u32) -> u32 {
    if index == len {
        hash
    } else {
        let new_hash = (hash.wrapping_mul(33)) ^ (bytes[index] as u32);
        hash_quick_str_const_rec(bytes, len, index + 1, new_hash)
    }
}

pub const fn hash_quick_str_const(str: &str) -> u32 {
    hash_quick_str_const_rec(str.as_bytes(), str.len(), 0, 5381)
}

pub fn hash_quick_str(str: &str) -> u32 {
    let mut hash = Wrapping(5381u32);
    for &datum in str.as_bytes() {
        hash = (hash * Wrapping(33)) ^ Wrapping(datum as u32);
    }
    hash.0
}

pub trait QuickHashExt {
    fn hash_quick(&self) -> u32;
}

impl QuickHashExt for str {
    fn hash_quick(&self) -> u32 {
        hash_quick_str(self)
    }
}

impl QuickHashExt for String {
    fn hash_quick(&self) -> u32 {
        hash_quick_str(self)
    }
}
