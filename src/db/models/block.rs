use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Block {
    pub id: i32,
    pub index: i32,
    pub previous_hash: Option<String>,
    pub timestamp: NaiveDateTime,
    pub nonce: Option<i32>,
    pub hash: Option<String>,
    pub data: Option<String>,
    pub miner_wallet_id: i32,
    pub difficulty: i32,
    pub tx_count: i32,
}
