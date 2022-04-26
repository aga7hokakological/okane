pub mod block;
use crate::block::Block;

pub struct Blockchain {
    blockchain: vec![]
}

impl Blockchain {
    pub fn new(blockchain: vec![]) -> Self {
        Self { blockchain }
    }

    pub fn get_latest_block() -> Block {
        return blockchain[blockchain.len() - 1]
    }

    pub fn generate_next_block(block_data: String) -> Block {
        let mut previous_block: Block = get_latest_block();
        let mut next_index: u64 = previous_block.index + 1;
        let mut timestamp: DateTime = Utc::now();
        let next_timestamp: u64 = timestamp.timestamp();
        let mut next_hash: String = calculate_hash(next_index, previous_block.hash, next_timestamp, block_data);
        let mut new_block: Block = Block::new(next_index, next_hash, previous_block.hash, next_timestamp, block_data);

        return new_block
    }
}