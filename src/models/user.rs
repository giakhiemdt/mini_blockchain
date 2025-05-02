use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInformationResponse {
    pub name: String,
    pub email: String,
    pub profile_pic_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_verified: Option<u8>, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserInformationRequest {
    pub email: Option<String>,
    pub password: Option<String>,
    pub profile_pic_url: Option<String>,
    pub is_active: Option<bool>,
    pub is_verified: Option<String>
}