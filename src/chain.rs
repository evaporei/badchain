use crate::block::Block;

pub struct Blockchain {
    blocks: Vec<Block>,
}

fn genesis() -> Block {
    Block::new("Genesis Block", &[])
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![genesis()],
        }
    }

    pub fn add_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new(data, &prev_block.hash);
        self.blocks.push(new_block);
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
}
