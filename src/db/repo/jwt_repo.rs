use sqlx::{Pool, MySql};
// use crate::db::models::jwt::Jwt;

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