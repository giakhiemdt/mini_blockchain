use dotenvy::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::env;

pub async fn connect_db() -> Pool<MySql> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db_url_no_db = env::var("DATABASE_URL_NO_DB").expect("DATABASE_URL_NO_DB not set");

    let pool_no_db = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url_no_db)
        .await
        .expect("Cannot connect to MySQL (no DB)!");

    println!("✅ Connected MySQL!");

    sqlx::query("CREATE DATABASE IF NOT EXISTS MiniBlockchain")
        .execute(&pool_no_db)
        .await
        .expect("Cannot create database!");

    println!("✅ Created database!");

    let pool  = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Cannot connect to MySQL (with DB)!");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations error!");

    println!("✅ Migrations ok!");

    pool

}



// async fn create_users_table(pool: &sqlx::Pool<sqlx::MySql>) {
//     let query = r#"
//         CREATE TABLE IF NOT EXISTS Users (
//             id INT AUTO_INCREMENT PRIMARY KEY,
//             name VARCHAR(255) NOT NULL UNIQUE,
//             password VARCHAR(255) NOT NULL,
//             email VARCHAR(255) NOT NULL UNIQUE,
//             role VARCHAR(50) DEFAULT 'user',
//             profile_pic_url VARCHAR(255),
//             created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
//             updated_at TIMESTAMP,
//             last_login TIMESTAMP,
//             is_active BIT DEFAULT true,
//             is_verified BIT DEFAULT false
//         );
//     "#;

//     exec(pool, query).await;
// }

// async fn create_jwt_table(pool: &sqlx::Pool<sqlx::MySql>) {
//     let query = r#"
//         CREATE TABLE IF NOT EXISTS Jwts (
//             id INT AUTO_INCREMENT PRIMARY KEY,
//             user_id INT NOT NULL,
//             token VARCHAR(512) NOT NULL UNIQUE,
//             exp DATETIME NOT NULL,
//             FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE
//         );
//     "#;

//     exec(pool, query).await;
// }


// async fn create_wallets_table(pool: &sqlx::Pool<sqlx::MySql>) {
//     let query = r#"
//         CREATE TABLE IF NOT EXISTS Wallets (
//             id INT AUTO_INCREMENT PRIMARY KEY,
//             user_id INT NOT NULL,
//             name VARCHAR(255) NOT NULL, 
//             balance DECIMAL(18, 8) NOT NULL DEFAULT 0,
//             created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
//             last_active_at TIMESTAMP,
//             address VARCHAR(255) NOT NULL,
//             is_active BIT DEFAULT true,
//             FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE
//         );
//     "#;

//     exec(pool, query).await;
// }

// async fn create_blocks_table(pool: &sqlx::Pool<sqlx::MySql>) {
//     let query = r#"
//         CREATE TABLE IF NOT EXISTS Blocks (
//             id INT AUTO_INCREMENT PRIMARY KEY,
//             `index` INT NOT NULL,
//             previous_hash VARCHAR(255),
//             timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
//             nonce INT,
//             hash VARCHAR(255),
//             data TEXT,
//             miner_wallet_id INT NOT NULL,
//             difficulty INT NOT NULL DEFAULT 4,
//             tx_count INT NOT NULL,
//             FOREIGN KEY (miner_wallet_id) REFERENCES Wallets(id) ON DELETE CASCADE
//         );
//     "#;

//     exec(pool, query).await;
// }

// async fn create_transactions_table(pool: &sqlx::Pool<sqlx::MySql>) {
//     let query = r#"
//         CREATE TABLE IF NOT EXISTS Transactions (
//             id INT AUTO_INCREMENT PRIMARY KEY,
//             sender_wallet_id INT NOT NULL,
//             receiver_wallet_id INT NOT NULL,
//             amount DECIMAL(18, 8) NOT NULL,
//             block_id INT,
//             timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
//             status VARCHAR(255) NOT NULL DEFAULT pending,
//             signature VARCHAR(255) NOT NULL,
//             tx_hash VARCHAR(255) NOT NULL,
//             fee FLOAT NOT NULL,
//             FOREIGN KEY (sender_wallet_id) REFERENCES Wallets(id) ON DELETE CASCADE,
//             FOREIGN KEY (receiver_wallet_id) REFERENCES Wallets(id) ON DELETE CASCADE,
//             FOREIGN KEY (block_id) REFERENCES Blocks(id) ON DELETE SET NULL
//         );
//     "#;

//     exec(pool, query).await;
// }

// async fn exec(pool: &sqlx::Pool<sqlx::MySql>, sql: &str) {
//     sqlx::query(sql)
//         .execute(pool)
//         .await
//         .expect("❌ Create table error!");
// }
