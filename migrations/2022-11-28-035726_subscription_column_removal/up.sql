-- Your SQL goes here
ALTER TABLE subscriptions
  DROP COLUMN merchant,
  DROP COLUMN paid,
  DROP COLUMN invoice_id,
  ADD COLUMN status varchar NOT NULL DEFAULT 'PENDING';
