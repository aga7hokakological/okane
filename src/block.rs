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

    // pub fn generate_next_block(block_data: String) {
    //     let mut previous_block: Block = get_latest_block();
    //     let mut next_index: u64 = previous_block.index + 1;
    //     let mut timestamp: DateTime<Tz> = Utc::now();
    //     let next_timestamp: u64 = timestamp.timestamp();
    //     let mut next_hash: String = calculate_hash(next_index, previous_block.hash, next_timestamp, block_data);
    // }
}