-- Your SQL goes here
ALTER TABLE users ADD COLUMN balance INT;
ALTER TABLE prompts ADD COLUMN total_token INT;
ALTER TABLE prompts ADD COLUMN total_cost INT;