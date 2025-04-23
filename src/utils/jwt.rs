use chrono::{Utc, Duration};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::{MySql, Pool};
use std::env;
use crate::models::jwt::Claims;

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

    let result = sqlx::query!(
        r#"
        INSERT INTO Jwts (user_id, token, exp)
        VALUES (?, ?, ?)
        "#,
        user_id,
        token,
        naive_exp,
    )
    .execute(&pool)
    .await;

    if let Err(err) = result {
        eprintln!("❌ Lỗi khi lưu JWT vào database: {:?}", err);
    }

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
    .map_err(|_| "Token không hợp lệ hoặc đã hết hạn".to_string())?;

    let claims = decoded.claims;

    let token_record = sqlx::query!(
        r#"
        SELECT id, user_id, exp FROM Jwts WHERE token = ?
        "#,
        token
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| "Lỗi truy vấn database khi kiểm tra token".to_string())?;

    let record = match token_record {
        Some(r) => r,
        None => return Err("Token không tồn tại hoặc đã bị thu hồi".to_string()),
    };

    let now = Utc::now().naive_utc();
    if record.exp < now {
        return Err("Token đã hết hạn".to_string());
    }

    Ok(claims)
}

