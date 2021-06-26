use crate::pow::{self, ProofOfWork};
use crate::transaction::Transaction;
use crate::utils::sha256;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
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
    pub fn new(transactions: Vec<Transaction>, prev_block_hash: &[u8]) -> Self {
        let block = Self {
            timestamp: unix_time(),
            transactions,
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

    pub fn hash_transactions(&self) -> Vec<u8> {
        let trx_ids_flat: Vec<u8> = self
            .transactions
            .iter()
            .flat_map(|trx| trx.id.clone())
            .collect();
        sha256(&trx_ids_flat)
    }
}
