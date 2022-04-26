// pub mod blockchain;
// use crate::blockchain::Blockchain;

use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: String,
}

impl Block {
    pub fn new(index: u64, hash: String, previous_hash: String, timestamp: u64, data: String) -> Self {
        Self { 
            index: index, 
            hash: hash, 
            previous_hash: previous_hash, 
            timestamp: timestamp,
            data: data,
        }
    }

    /// Genesis block
    // pub fn genesis_block(&mut self) {
    //     let genesis_block = Block {
    //         index: 0,
    //         hash: String::from("816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7"), 
    //         previous_hash: String::from("0"), 
    //         timestamp: 1465154705, 
    //         data: String::from("my genesis block!!"),
    //     }
    // }

    pub fn calculate_hash(
        &self, 
        index: u64, 
        previous_hash: String, 
        timestamp: u64, 
        data: String
    ) -> Vec<u8> {
            let block_data = serde_json::json!({
                "index": index,
                "previous_hash": previous_hash,
                "timestamp": timestamp,
                "data": data
            });
            let mut hash = Sha256::new();
            hash.update(block_data.to_string().as_bytes());
            
        return hash.finalize().as_slice().to_owned()
    }

    // pub fn is_valid_block(new_block: Block, previous_block: Block) -> bool {
    //     if (previous_block.index + 1 != new_block.index) {
    //         panic!("Invalid block")
    //         // return false
    //     } else if (previous_block.hash != new_block.previous_hash) {
    //         panic!("Invalid previous hash")
    //         // return false
    //     } else if (calculate_hash_for_block(new_block) != new_block.hash) {
    //         println!("{:?} + {:?}", new_block.hash, calculate_hash_for_block(new_block));
    //         println!("Invalid hash {:?}", new_block.hash);
    //         return false
    //     }

    //     return true
    // }

    // pub fn calculate_hash_for_block(block: Block) -> Vec<u8> {
    //     calculate_hash(block.index, block.previous_hash, block.timestamp, block.data)
    // }
}