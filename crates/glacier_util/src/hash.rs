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

pub fn hash_quick_str(str: &str) -> u32 {
    let mut hash = Wrapping(5381u32);
    for &datum in str.as_bytes() {
        hash = (hash * Wrapping(33)) ^ Wrapping(datum as u32);
    }
    hash.0
}

pub trait QuickHashExt {
    fn quick_hash(&self) -> u32;
}

impl QuickHashExt for str {
    fn quick_hash(&self) -> u32 {
        hash_quick_str(self)
    }
}

impl QuickHashExt for String {
    fn quick_hash(&self) -> u32 {
        hash_quick_str(self)
    }
}
