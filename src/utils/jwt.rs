use chrono::{Utc, Duration};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::{MySql, Pool};
use std::env;
use crate::models::jwt::Claims;
use crate::db::repo::jwt_repo;

pub async fn create_jwt(
    user_id: &str, 
    user_email: &str, 
    user_role: &str,
    pool: Pool<MySql>
) -> String {
    let expiration_time = Utc::now() + Duration::hours(1); // hết hạn sau 1 giờ
    let expiration_timestamp = expiration_time.timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration_timestamp,
        email: user_email.to_string(),
        role: user_role.to_string(),
    };

    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not set");

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .expect("Failed to create JWT");

    let naive_exp = expiration_time.naive_utc();

    jwt_repo::create_jwt(&pool, user_id, token.clone(), naive_exp)
    .await.expect("Không thể lưu JWT vào cơ sở dữ liệu");

    token
}

pub async fn validate_jwt(
    token: &str,
    pool: Pool<MySql>,
) -> Result<Claims, String> {

    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not set");

    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| "Invalid token or exprired!".to_string())?;

    let claims = decoded.claims;

    let record = jwt_repo::find_token_by_token(&pool, token.to_string())
        .await.expect("Failed to find token in database!");

    let now = Utc::now().naive_utc();
    if record.exp < now {
        return Err("Token exprired!".to_string());
    }

    Ok(claims)
}

