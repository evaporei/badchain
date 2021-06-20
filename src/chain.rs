use crate::block::Block;
use anyhow::Context;

pub struct Blockchain {
    last_block_hash: Vec<u8>,
    db: sled::Db,
}

fn create_genesis() -> Block {
    Block::new("Genesis Block", &[])
}

const DATABASE_FILE_NAME: &str = "/tmp/badchain-db";

impl Blockchain {
    pub fn new() -> anyhow::Result<Self> {
        let db = sled::open(DATABASE_FILE_NAME).context("Failed to open database file")?;

        let last_block_hash = match db
            .get("last_block_hash")
            .context("Failed to get last hash from database")?
        {
            Some(last_block_hash) => last_block_hash.to_vec(),
            _ => {
                let genesis = create_genesis();
                db.insert(
                    genesis.hash.clone(),
                    bincode::serialize(&genesis).context("Failed to serialize genesis block")?,
                )
                .context("Failed to insert genesis block in database")?;
                db.insert("last_block_hash", genesis.hash.clone())
                    .context("Failed to insert last block hash in database")?;
                genesis.hash
            }
        };

        Ok(Self {
            last_block_hash,
            db,
        })
    }

    pub fn add_block(&mut self, data: &str) -> anyhow::Result<()> {
        let new_block = Block::new(data, &self.last_block_hash);
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
