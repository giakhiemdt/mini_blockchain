use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Transaction {
    pub id: i32,
    pub sender_wallet_id: i32,
    pub receiver_wallet_id: i32,
    pub amount: BigDecimal,
    pub block_id: Option<i32>,
    pub timestamp: NaiveDateTime,
    pub status: String,
    pub signature: String,
    pub tx_hash: String,
    pub fee: f32,
    pub nonce: i32,
}
