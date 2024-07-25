-- Your SQL goes here
ALTER TABLE subscriptions DROP COLUMN topup_id;
ALTER TABLE subscriptions DROP COLUMN is_paylater;
ALTER TABLE subscriptions ADD COLUMN price FLOAT NOT NULL DEFAULT 0.0;