-- This file should undo anything in `up.sql`
ALTER TABLE users ADD COLUMN balance FLOAT NOT NULL DEFAULT 0.0;
ALTER TABLE prompts ADD COLUMN total_cost FLOAT NOT NULL DEFAULT 0.0;