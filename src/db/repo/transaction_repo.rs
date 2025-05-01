use bigdecimal::BigDecimal;
use sqlx::{MySql, Pool};

pub async fn insert_new_transaction(
    pool: &Pool<MySql>,
    sender_wallet_id: &str, 
    receiver_wallet_id: &str, 
    amount: BigDecimal, 
    timestamp: &str, 
    signature: &str, 
    tx_hash: &str, 
    fee: BigDecimal,
    nonce: i64
) -> Result<u64, sqlx::Error> {

    let result = sqlx::query!(
        r#"
        INSERT INTO Transactions (sender_wallet_id, receiver_wallet_id, amount, 
        timestamp, signature, tx_hash, fee, nonce)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        sender_wallet_id,
        receiver_wallet_id,
        amount,
        timestamp,
        signature,
        tx_hash,
        fee,
        nonce
    )
    .execute(pool)
    .await?;
    
    Ok(result.last_insert_id())
}

pub async fn get_current_nonce(
    pool: &Pool<MySql>,
    sender_wallet_id: &str
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "count!: i64" FROM Transactions WHERE sender_wallet_id = ?
        "#,
        sender_wallet_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}
