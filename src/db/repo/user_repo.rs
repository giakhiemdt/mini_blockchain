use sqlx::{Pool, MySql};
use crate::db::models::user::{User, UserBasic};

pub async fn find_user_by_username_and_password(
    pool: &Pool<MySql>,
    username: &str,
    password: &str
) -> Result<UserBasic, sqlx::Error> {
    let user = sqlx::query_as!(
        UserBasic,
        r#"
        SELECT 
            id,
            name,
            email,
            role
        FROM Users
        WHERE name = ? AND password = ?
        "#,
        username,
        password
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}


pub async fn find_user_by_id(
    pool: &Pool<MySql>,
    id: &str
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id,
            name,
            password,
            email,
            role,
            profile_pic_url,
            created_at,
            updated_at,
            last_login,
            is_active,
            is_verified
        FROM Users WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn insert_new_user(
    pool: &Pool<MySql>,
    name: &str,
    email: &str,
    password: &str
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO Users (name, email, password)
        VALUES (?, ?, ?)
        "#,
        name,
        email,
        password,
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_id())
}
