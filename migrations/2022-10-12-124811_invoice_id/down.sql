-- This file should undo anything in `up.sql`
ALTER TABLE subscriptions
  DROP COLUMN invoice_id;