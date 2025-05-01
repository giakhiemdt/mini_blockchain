use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;



#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWalletRequest {
    pub wallet_name: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct GetWalletInformationRequest {
    pub wallet_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetWalletInformationResponse {
    pub name: String,
    pub balance: BigDecimal,
    pub created_at: Option<DateTime<Utc>>,
    pub address: String,
    pub public_key: String,
}