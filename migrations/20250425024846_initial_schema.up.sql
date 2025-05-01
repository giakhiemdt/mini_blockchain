-- Add up migration script here

ALTER TABLE Transactions ADD COLUMN nonce INT NOT NULL;
