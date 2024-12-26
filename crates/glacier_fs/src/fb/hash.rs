use std::num::Wrapping;

pub fn hash_with_seed(data: &[u8], hash: u64) -> u64 {
    let mut hash = Wrapping(hash);
    for &datum in data {
        hash = hash * Wrapping(33) ^ Wrapping(datum as u64);
    }
    hash.0
}

pub fn hash_string(str: &str) -> u64 {
    let l = hash_with_seed(str.to_lowercase().as_bytes(), 5381);
    l & 0xFFFFFFFF
}
