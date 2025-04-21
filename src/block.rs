use chrono::Utc;
use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub data: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new (index: u32, data: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let block = Block {
            index,
            timestamp: timestamp.clone(),
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        // block.hash = block.calculate_hash();
        block
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        loop {
            self.hash = self.calculate_hash();
            if &self.hash[..difficulty] == target {
                println!(
                    "✅ Mined successfully! Nonce: {}, Hash: {}",
                    self.nonce, self.hash
                );
                break;
            }
    
            if self.nonce % 100_000 == 0 {
                println!("⛏️  Mining... nonce = {}, hash = {}", self.nonce, self.hash);
            }
    
            self.nonce += 1;
        }
    }

    pub fn calculate_hash (&self) -> String {
        let data_json = serde_json::to_string(&self.data).unwrap();

        let block_content = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, data_json, self.previous_hash, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(block_content.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}