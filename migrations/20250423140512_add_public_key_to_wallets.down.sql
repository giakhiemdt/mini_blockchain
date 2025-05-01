-- Add down migration script here
ALTER TABLE Wallets DROP COLUMN public_key;
