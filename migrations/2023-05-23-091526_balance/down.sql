-- This file should undo anything in `up.sql`
ALTER TABLE users DROP COLUMN balance;
ALTER TABLE prompts DROP COLUMN total_token;
ALTER TABLE prompts DROP COLUMN total_cost;