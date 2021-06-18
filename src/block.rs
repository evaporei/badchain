use sha2::digest::Digest;
use sha2::Sha256;
use std::time::SystemTime;

pub struct Block {
    timestamp: u64,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
}

fn unix_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl Block {
    pub fn new(data: &str, prev_block_hash: &[u8]) -> Self {
        let mut block = Self {
            timestamp: unix_time(),
            data: data.bytes().collect(),
            prev_block_hash: prev_block_hash.into(),
            hash: vec![],
        };

        block.set_hash();

        block
    }

    fn set_hash(&mut self) {
        let timestamp: Vec<u8> = self.timestamp.to_string().bytes().collect();

        let mut hasher = Sha256::new();
        hasher.update(&self.prev_block_hash);
        hasher.update(&self.data);
        hasher.update(&timestamp);
        let hash = hasher.finalize()[..].to_vec();

        self.hash = hash;
    }
}
