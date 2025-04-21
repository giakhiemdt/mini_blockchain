mod block;
mod blockchain;
mod transaction;
// use block::Block;
use blockchain::Blockchain;
use crate::transaction::Transaction;

fn main() {

    let mut my_blockchain = Blockchain::new(5);

    my_blockchain.add_transaction(Transaction {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 50.0,
    });

    my_blockchain.add_transaction(Transaction {
        from: "Bob".to_string(),
        to: "Charlie".to_string(),
        amount: 25.0,
    });

    my_blockchain.mine_pendding_transaction();

    for block in my_blockchain.chain.iter() {
        println!("Block {}: {:?}", block.index, block);
    }

    println!("Blockchain is valid? {}", my_blockchain.is_valid());
}