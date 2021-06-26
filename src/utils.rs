use sha2::digest::Digest;
use sha2::Sha256;

pub fn hash_to_str(hash_bytes: &[u8]) -> String {
    let mut output = String::new();
    for byte in hash_bytes {
        output.push_str(&format!("{:x}", *byte));
    }
    output
}

pub fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize()[..].to_vec()
}
