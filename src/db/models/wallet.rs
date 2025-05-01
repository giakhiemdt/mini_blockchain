use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Wallet {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub balance: BigDecimal,
    pub created_at: Option<DateTime<Utc>>,
    pub last_active_at: Option<DateTime<Utc>>,
    pub address: String,
    pub is_active: Option<u8>,
    pub public_key: String,
}

#[derive(Debug, FromRow)]
pub struct WalletBasic {
    pub name: String,
    pub balance: BigDecimal,
    pub created_at: Option<DateTime<Utc>>,
    pub address: String,
    pub public_key: String,
}
