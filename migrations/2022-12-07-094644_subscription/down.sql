-- This file should undo anything in `up.sql`
ALTER TABLE subscriptions
  ADD COLUMN status varchar NOT NULL DEFAULT 'PENDING',
  ADD COLUMN channels_name varchar NOT NULL,
  ADD COLUMN channels_slug varchar NOT NULL;
