// pub mod simplechain;
use crate::blockchain::block::Block;


use chrono::{Utc, DateTime};

pub struct SimpleChain {
    simple_chain: Vec<Block>,
}

impl SimpleChain {
    pub fn new(blockchain: Vec<Block>) -> Self {
        SimpleChain { simple_chain: blockchain }
    }

    pub fn get_latest_block(&self) -> Block {
        let block: Block = self.simple_chain[self.simple_chain.len() - 1];
        return block
    }
}