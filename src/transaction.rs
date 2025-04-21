use serde::{Serialize, Deserialize};
// use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
}

impl Transaction {
    
    // pub fn new(from: String, to: String, amount: f64) -> Self{
    //     let transaction = Transaction {
    //         from,
    //         to,
    //         amount
    //     };
    //     transaction
    // }

}