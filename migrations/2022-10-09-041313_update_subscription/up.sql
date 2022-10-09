-- Your SQL goes here
ALTER TABLE subscriptions
  DROP COLUMN status,
  DROP COLUMN monthly_price,
  DROP COLUMN total_price,
  ADD COLUMN paid BOOLEAN NOT NULL DEFAULT FALSE,
  ADD COLUMN duration integer NOT NULL DEFAULT 1;