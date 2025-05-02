use chrono::NaiveDateTime;
use sqlx::{Pool, MySql};

use crate::db::models::jwt::Jwt;
// use crate::db::models::jwt::Jwt;

pub async fn create_jwt(
    pool: &Pool<MySql>,
    user_id: &str,
    token: String,
    naive_exp: NaiveDateTime,
) -> Result<(), sqlx::Error> {

    sqlx::query!(
        r#"
        INSERT INTO Jwts (user_id, token, exp)
        VALUES (?, ?, ?)
        "#,
        user_id,
        token,
        naive_exp,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_token_by_token(
    pool: &Pool<MySql>,
    token: String
) -> Result<Jwt, sqlx::Error> {

    let token_record = sqlx::query_as!(
        Jwt,
        r#"
        SELECT id, user_id, token, exp FROM Jwts WHERE token = ?
        "#,
        token
    )
    .fetch_one(pool)
    .await?;

    Ok(token_record)
}

pub async fn delete_jwt(
    pool: &Pool<MySql>,
    user_id: &str
) -> Result<(), sqlx::Error> {

    sqlx::query!(
        r#"
        DELETE FROM Jwts WHERE user_id = ?
        "#,
        user_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}