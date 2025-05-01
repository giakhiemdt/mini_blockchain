use sqlx::{MySql, Pool};

use crate::db::models::wallet::WalletBasic;

pub async fn insert_new_wallet(
    pool: &Pool<MySql>,
    user_id: &str,
    name: &str,
    address: &str,
    public_key: &str
) -> Result<u64, sqlx::Error> {

    let result = sqlx::query!(
        r#"
        INSERT INTO Wallets (user_id, name, address, public_key)
        VALUES (?, ?, ?, ?)
        "#,
        user_id,
        name,
        address,
        public_key
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_id())
}

pub async fn find_wallet_by_id_and_userid(
    pool: &Pool<MySql>,
    id: &str,
    userid: &str,
) -> Result<WalletBasic, sqlx::Error> {

    let result = sqlx::query_as!(
        WalletBasic,
        r#"
        SELECT name, balance, created_at, address, public_key 
        FROM Wallets 
        WHERE id = ? AND user_id = ?
        "#,
        id,
        userid
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn find_wallet_id_by_address(
    pool: &Pool<MySql>,
    address: &str
) -> String {
    match sqlx::query!(
        r#"
        SELECT id FROM Wallets WHERE address = ?
        "#,
        address
    )
    .fetch_one(pool)
    .await
    {
        Ok(wallet) => wallet.id.to_string(),
        Err(_) => String::new(),
    } 

    
}

pub async fn find_wallet_public_key_by_address(
    pool: &Pool<MySql>,
    address: &str
) -> String {
    match sqlx::query!(
        r#"
        SELECT public_key FROM Wallets WHERE address = ?
        "#,
        address
    )
    .fetch_one(pool)
    .await
    {
        Ok(wallet) => wallet.public_key,
        Err(_) => String::new(),
    }
}

pub async fn check_wallet_owner(
    pool: &Pool<MySql>,
    address: &str,
    user_id: &str
) -> bool {
    match sqlx::query!(
        r#"
        SELECT id FROM Wallets WHERE address = ? AND user_id = ?
        "#,
        address,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(_) => true,
        Err(_) => false
    }
}