use crate::block::Block;
use crate::transaction::Transaction;
use crate::utils::hash_to_str;
use anyhow::Context;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Blockchain {
    last_block_hash: Vec<u8>,
    db: sled::Db,
}

const GENESIS_COINBASE_DATA: &str =
    "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

pub fn create_genesis(coinbase: Transaction) -> Block {
    Block::new(vec![coinbase], &[])
}

pub const DATABASE_FILE_NAME: &str = "/tmp/badchain-db";

pub fn setup_database(address: &str) -> anyhow::Result<()> {
    let db = sled::open(DATABASE_FILE_NAME).context("Failed to open database file")?;

    let coinbase_trx = Transaction::new_coinbase(address, GENESIS_COINBASE_DATA);
    let genesis = create_genesis(coinbase_trx);
    db.insert(
        genesis.hash.clone(),
        bincode::serialize(&genesis).context("Failed to serialize genesis block")?,
    )
    .context("Failed to insert genesis block in database")?;
    db.insert("last_block_hash", genesis.hash)
        .context("Failed to insert last block hash in database")?;

    Ok(())
}

impl Blockchain {
    pub fn new() -> anyhow::Result<Self> {
        let db = sled::open(DATABASE_FILE_NAME).context("Failed to open database file")?;

        let last_block_hash = db
            .get("last_block_hash")
            .context("Failed to get last hash from database")?
            .unwrap() // safe because fn setup_database is called first
            .to_vec();

        Ok(Self {
            last_block_hash,
            db,
        })
    }

    pub fn mine_block(&mut self, transactions: Vec<Transaction>) -> anyhow::Result<()> {
        let new_block = Block::new(transactions, &self.last_block_hash);
        let serialized = bincode::serialize(&new_block).context("Failed to serialize new block")?;

        let mut db_batch = sled::Batch::default();
        db_batch.insert(new_block.hash.clone(), serialized);
        db_batch.insert("last_block_hash", new_block.hash.clone());
        self.db
            .apply_batch(db_batch)
            .context("Failed to add new block and last block hash to database")?;

        self.last_block_hash = new_block.hash;

        Ok(())
    }

    pub fn find_unspent_transactions(&self, address: &str) -> anyhow::Result<Vec<Transaction>> {
        let mut unspent_trxs = vec![];
        let mut spent_trxs: HashMap<String, Vec<isize>> = HashMap::new();
        let chain_it = self.clone().into_iter(); // cheap clone, the data is all in the database

        for block in chain_it {
            let block = block?;
            for trx in block.transactions {
                let trx_id = hash_to_str(&trx.id);

                'outputs: loop {
                    for (output_idx, output) in trx.outputs.iter().enumerate() {
                        // was the output sent?
                        if let Some(spent_outputs) = spent_trxs.get(&trx_id) {
                            for spent_output in spent_outputs.iter() {
                                if *spent_output == output_idx as isize {
                                    continue 'outputs;
                                }
                            }
                        }

                        if output.can_be_unlocked_with(address) {
                            unspent_trxs.push(trx.clone());
                        }
                    }

                    if !trx.is_coinbase() {
                        for input in trx.inputs.iter() {
                            if input.can_unlock_output_with(address) {
                                let input_trx_id = hash_to_str(&input.trx_id);
                                spent_trxs.get_mut(&input_trx_id).map(|spent_trx| {
                                    spent_trx.push(input.output_idx);
                                });
                            }
                        }
                    }
                }
            }

            if block.prev_block_hash.is_empty() {
                break;
            }
        }

        Ok(unspent_trxs)
    }
}

impl IntoIterator for Blockchain {
    type Item = anyhow::Result<Block>;
    type IntoIter = BlockchainIterator;

    fn into_iter(self) -> Self::IntoIter {
        BlockchainIterator {
            current_hash: self.last_block_hash,
            db: self.db,
        }
    }
}

pub struct BlockchainIterator {
    current_hash: Vec<u8>,
    db: sled::Db,
}

impl Iterator for BlockchainIterator {
    type Item = anyhow::Result<Block>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.db.get(&self.current_hash) {
            Ok(Some(serialized_block)) => match bincode::deserialize::<Block>(&serialized_block) {
                Ok(deserialized_block) => {
                    self.current_hash = deserialized_block.prev_block_hash.clone();
                    Some(Ok(deserialized_block))
                }
                Err(err) => {
                    Some(Err(err).context("Failed to deserialize last block from database"))
                }
            },
            Ok(None) => None,
            Err(err) => Some(Err(err).context("Failed to get last block from database")),
        }
    }
}
