-- Add up migration script here
ALTER TABLE Wallets ADD COLUMN public_key VARCHAR(255) NOT NULL;
