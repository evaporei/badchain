use badchain::chain::Blockchain;
use badchain::pow::ProofOfWork;
use std::str::from_utf8;

fn hash_to_str(hash_bytes: &[u8]) -> String {
    let mut output = String::new();
    for byte in hash_bytes {
        output.push_str(&format!("{:x}", *byte));
    }
    output
}

fn main() {
    let mut chain = Blockchain::new();

    chain.add_block("Send 1 BTC to Ivan");
    chain.add_block("Send 2 more BTC to Ivan");

    for block in chain.blocks {
        println!("Prev. hash: {:?}", block.prev_block_hash);
        println!("Data: {}", from_utf8(&block.data).unwrap());
        println!("Hash: {}", hash_to_str(&block.hash));
        let pow = ProofOfWork::new(block);
        println!("PoW: {}", pow.validate());
    }
}
