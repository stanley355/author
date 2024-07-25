-- This file should undo anything in `up.sql`
ALTER TABLE subscriptions ADD COLUMN topup_id uuid REFERENCES topups(id);
ALTER TABLE subscriptions ADD COLUMN is_paylater boolean DEFAULT false;
ALTER TABLE subscriptions DROP COLUMN price;
