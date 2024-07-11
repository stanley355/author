-- This file should undo anything in `up.sql`
ALTER TABLE users DROP COLUMN updated_at;
ALTER TABLE prompts DROP COLUMN updated_at;
ALTER TABLE topups DROP COLUMN updated_at;
ALTER TABLE subscriptions DROP COLUMN updated_at;