use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTransactionRequest {
    pub tx_data: TxDataRequest,
    pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TxDataRequest {
    pub sender_wallet_address: String,
    pub receiver_wallet_address: String,
    pub amount: BigDecimal,
    pub nonce: i64,
}