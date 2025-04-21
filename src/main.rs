mod block;
mod blockchain;
mod transaction;
// use block::Block;
use blockchain::Blockchain;
use crate::transaction::Transaction;

fn main() {

    // let mut my_blockchain = Blockchain::new(4);
    let mut my_blockchain = Blockchain::load_from_file("blockchain.json");
    println!("Loaded chain length: {}", my_blockchain.chain.len());

    my_blockchain.add_transaction(Transaction {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 70.0,
    });

    my_blockchain.add_transaction(Transaction {
        from: "Bob".to_string(),
        to: "Charlie".to_string(),
        amount: 25.0,
    });

    my_blockchain.mine_pendding_transaction();

    my_blockchain.save_to_file("blockchain.json");

    println!("Blockchain is valid? {}", my_blockchain.is_valid());

    println!("Balance of KhiemGia: {}", my_blockchain.get_balance("KhiemGia"));
    println!("Balance of Alice: {}", my_blockchain.get_balance("Alice"));
    println!("Balance of Bob: {}", my_blockchain.get_balance("Bob"));
    
}