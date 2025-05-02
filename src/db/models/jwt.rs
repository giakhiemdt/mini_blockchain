use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Jwt {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub exp: NaiveDateTime,
}
