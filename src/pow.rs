use crate::block::Block;
use sha2::digest::Digest;
use sha2::Sha256;
use std::convert::TryInto;

const TARGET_BITS: u64 = 18;
const MAX_NONCE: u64 = u64::MAX;

pub struct ProofOfWork {
    pub block: Block,
    target: u64,
}

pub struct Data {
    pub nonce: u64,
    pub hash: Vec<u8>,
}

pub fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize()[..].to_vec()
}

impl ProofOfWork {
    pub fn new(block: Block) -> Self {
        let target: u64 = 1;
        let target = target.wrapping_shl((256u64 - TARGET_BITS).try_into().unwrap());

        Self { block, target }
    }

    fn prepare_data(&self, nonce: u64) -> Vec<u8> {
        self.block
            .prev_block_hash
            .iter()
            .chain(self.block.data.iter())
            .cloned()
            .chain(self.block.timestamp.to_string().bytes())
            .chain(TARGET_BITS.to_string().bytes())
            .chain(nonce.to_string().bytes())
            .collect()
    }

    pub fn run(&self) -> Data {
        let mut hash_int;
        let mut nonce = 0;
        let mut hash: Vec<u8> = vec![];

        println!(
            "Mining the block containing {}",
            std::str::from_utf8(&self.block.data).unwrap()
        );

        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = sha256(&data);
            // println!("{:?}", hash);

            let mut arr: [u8; 8] = [0; 8];
            arr.copy_from_slice(&hash[0..8]);
            hash_int = u64::from_be_bytes(arr);

            if hash_int < self.target {
                break;
            } else {
                nonce += 1;
            }
        }

        Data { nonce, hash }
    }

    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        let hash = sha256(&data);

        let mut arr: [u8; 8] = [0; 8];
        arr.copy_from_slice(&hash[0..8]);
        let hash_int = u64::from_be_bytes(arr);

        hash_int < self.target
    }
}
