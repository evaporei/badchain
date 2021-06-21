use crate::pow::{self, ProofOfWork};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub timestamp: u64,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: u64,
}

fn unix_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl Block {
    pub fn new(data: &str, prev_block_hash: &[u8]) -> Self {
        let block = Self {
            timestamp: unix_time(),
            data: data.bytes().collect(),
            prev_block_hash: prev_block_hash.into(),
            hash: vec![],
            nonce: 0,
        };

        let pow = ProofOfWork::new(block);

        let pow::Data { nonce, hash } = pow.run();

        let ProofOfWork { mut block, .. } = pow;

        block.hash = hash;
        block.nonce = nonce;

        block
    }
}
