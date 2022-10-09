-- This file should undo anything in `up.sql`
ALTER TABLE subscriptions
  DROP COLUMN paid,
  DROP COLUMN duration,
  ADD COLUMN monthly_price INTEGER NOT NULL,
  ADD COLUMN total_price INTEGER NOT NULL,
  ADD COLUMN status VARCHAR DEFAULT 'Pending';