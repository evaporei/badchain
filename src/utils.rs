pub fn hash_to_str(hash_bytes: &[u8]) -> String {
    let mut output = String::new();
    for byte in hash_bytes {
        output.push_str(&format!("{:x}", *byte));
    }
    output
}
