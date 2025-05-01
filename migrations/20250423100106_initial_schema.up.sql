-- Add up migration script here
CREATE TABLE IF NOT EXISTS Users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    role VARCHAR(50) DEFAULT 'user',
    profile_pic_url VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    last_login TIMESTAMP,
    is_active BIT DEFAULT true,
    is_verified BIT DEFAULT false
);

CREATE TABLE IF NOT EXISTS Jwts (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    token VARCHAR(512) NOT NULL UNIQUE,
    exp DATETIME NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Wallets (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    name VARCHAR(255) NOT NULL, 
    balance DECIMAL(18, 8) NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_active_at TIMESTAMP,
    address VARCHAR(255) NOT NULL,
    is_active BIT DEFAULT true,
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Blocks (
    id INT AUTO_INCREMENT PRIMARY KEY,
    `index` INT NOT NULL,
    previous_hash VARCHAR(255),
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    nonce INT,
    hash VARCHAR(255),
    data TEXT,
    miner_wallet_id INT NOT NULL,
    difficulty INT NOT NULL DEFAULT 4,
    tx_count INT NOT NULL,
    FOREIGN KEY (miner_wallet_id) REFERENCES Wallets(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Transactions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    sender_wallet_id INT NOT NULL,
    receiver_wallet_id INT NOT NULL,
    amount DECIMAL(18, 8) NOT NULL,
    block_id INT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(255) NOT NULL DEFAULT 'Pending',
    signature VARCHAR(255) NOT NULL,
    tx_hash VARCHAR(255) NOT NULL,
    fee FLOAT NOT NULL,
    FOREIGN KEY (sender_wallet_id) REFERENCES Wallets(id) ON DELETE CASCADE,
    FOREIGN KEY (receiver_wallet_id) REFERENCES Wallets(id) ON DELETE CASCADE,
    FOREIGN KEY (block_id) REFERENCES Blocks(id) ON DELETE SET NULL
);