use serde::{Deserialize, Serialize};

use crate::block::Block;
use crate::transaction::Transaction;
use std::fs::File;
use std::io::BufReader;

// use serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub difficulty: usize,
}

impl Blockchain {
    // pub fn new(difficulty: usize) -> Self {
    //     let mut blockchain = Blockchain {
    //         chain: Vec::new(),
    //         pending_transactions: Vec::new(),
    //         difficulty,
    //     };
    //     blockchain.create_genesis_block();
    //     blockchain
    // }

    // pub fn create_genesis_block(&mut self) {
    //     let genesis_block = Block::new(0,
    //         vec![Transaction::new("System".to_string(), "KhiemGia".to_string(), 50.00)],
    //          "0".to_string());
    //     self.chain.push(genesis_block);
    // }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        let tx = transaction.clone();

        if transaction.from != "SYSTEM" {
            let balance = self.get_balance(&transaction.from);
    
            if transaction.amount > balance {
                println!(
                    "❌ Transaction denied: '{}' does not have enough balance (has {}, needs {}).",
                    transaction.from, balance, transaction.amount
                );
                return;
            }
        }
    
        self.pending_transactions.push(transaction);
        println!(
            "✅ Transaction accepted: {} -> {} : {}",
            tx.from, tx.to, tx.amount
        );
    }

    pub fn mine_pendding_transaction(&mut self) {
        let previous_block = self.chain.last().unwrap();

        let mut transaction_data = self.pending_transactions.clone();

        transaction_data.push(
            Transaction { 
                from: "System".to_string(), 
                to: "KhiemGia".to_string(), 
                amount: 1.0,
            }
        );

        let mut block = Block::new(
            previous_block.index + 1,
            transaction_data,
            previous_block.hash.clone(),
        );

        block.mine_block(self.difficulty);

        self.pending_transactions.clear();

        self.chain.push(block);
    }

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

    pub fn save_to_file(&self, filename: &str) {
        let file = File::create(filename).expect("Unable to create file");
        serde_json::to_writer_pretty(file, &self).expect("Unable to write data");
        println!("Blockchain saved to file.");
    }

    pub fn load_from_file(filename: &str) -> Self {
        let file = File::open(filename).expect("Unable to open file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Unable to parse file")
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        let mut balance = 0.0;
        for block in &self.chain {
            for tx in &block.data {
                if tx.from == address {
                    balance -= tx.amount;
                }
                if tx.to == address {
                    balance += tx.amount;
                }
            }
        }
        balance
    }

}