use crate::block::Block;
use crate::transaction::{self, Transaction};
// use std::fs::File;
// use std::io::{Read, Write};
// use serde_json;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    pub fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0,
            vec![Transaction::new("System".to_string(), "KhiemGia".to_string(), 50.00)],
             "0".to_string());
        self.chain.push(genesis_block);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pendding_transaction(&mut self) {
        let previous_block = self.chain.last().unwrap();

        let mut block = Block::new(
            previous_block.index + 1,
            self.pending_transactions.clone(),
            previous_block.hash.clone(),
        );

        block.mine_block(self.difficulty);
        self.chain.push(block);
    }

    // pub fn add_block(&mut self, data: String) {
    //     let previous_block = self.chain.last().unwrap();
    //     let mut new_block = Block::new(
    //         previous_block.index + 1,
    //         data,
    //         previous_block.hash.clone(),
    //     );
    //     new_block.mine_block(4);
    //     self.chain.push(new_block)
    // }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }

        return true;
    }

    // pub fn save_to_file(&self, filename: &str) {
    //     let file = File::create(filename).expect("Unable to create file");
    //     serde_json::to_writer(file, &self.chain).expect("Unable to write data");
    //     println!("Blockchain saved to file.");
    // }

    // pub fn load_from_file(filename: &str) -> Self {
    //     let mut file = File::open(filename).expect("Unable to open file");
    //     let mut content = String::new();
    //     file.read_to_string(&mut content).expect("Unable to read data from file");

    //     let chain: Vec<Block> = serde_json::from_str(&content).expect("Unable to parse JSON");
    //     Blockchain { chain }
    // }
}