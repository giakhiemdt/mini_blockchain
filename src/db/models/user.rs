use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
    pub role: Option<String>,
    pub profile_pic_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: Option<u8>, 
    pub is_verified: Option<u8>, 
}


#[derive(Debug, FromRow)]
pub struct UserBasic {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: Option<String>,
}