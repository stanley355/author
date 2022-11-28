-- This file should undo anything in `up.sql`
ALTER TABLE subscriptions 
  ADD COLUMN invoice_id varchar NOT NULL,
  ADD COLUMN paid boolean NOT NULL DEFAULT false,
  ADD COLUMN merchant varchar NOT NULL DEFAULT 'kontenku';