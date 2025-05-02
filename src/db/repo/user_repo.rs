use chrono::{Utc, DateTime};
use sqlx::{Pool, MySql, QueryBuilder};
use crate::db::models::user::{User, UserBasic};

// Login api
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

// Register api
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

// Update user information
pub async fn update_user_information(
    pool: &Pool<MySql>,
    user_id: &str,
    email: Option<&str>,
    password: Option<&str>,
    profile_pic_url: Option<&str>,
    last_login: Option<DateTime<Utc>>,
    is_active: Option<bool>,
    is_verified: Option<&str>
) -> Result<(), sqlx::Error> {

    let mut qb = QueryBuilder::<MySql>::new("UPDATE Users SET ");

    let mut separated = qb.separated(", ");

    if let Some(email) = email {
        separated.push("email = ").push_bind(email);
    }
    if let Some(password) = password {
        separated.push("password = ").push_bind(password);
    }
    if let Some(url) = profile_pic_url {
        separated.push("profile_pic_url = ").push_bind(url);
    }
    if let Some(last_login) = last_login {
        separated.push("last_login = ").push_bind(last_login);
    }
    if let Some(is_active) = is_active {
        separated.push("is_active = ").push_bind(is_active);
    }
    if let Some(is_verified) = is_verified {
        separated.push("is_verified = ").push_bind(is_verified);
    }

    let now = Utc::now();
    separated.push("updated_at = ").push_bind(now);


    qb.push(" WHERE id = ").push_bind(user_id);

    let query = qb.build();
    query.execute(pool).await?;

    Ok(())
}
