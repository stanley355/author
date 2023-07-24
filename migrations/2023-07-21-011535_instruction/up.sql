-- Your SQL goes here
ALTER TABLE prompts ADD COLUMN instruction VARCHAR NOT NULL DEFAULT '';
ALTER TABLE prompts ADD COLUMN document_id uuid REFERENCES documents(id);