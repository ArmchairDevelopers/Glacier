pub fn hash_quick_str(data: &str) -> u32 {
    let mut hash = 5381;
    for byte in data.bytes() {
        hash = hash * 33 ^ byte as u32;
    }
    hash
}
