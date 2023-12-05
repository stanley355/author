-- This file should undo anything in `up.sql`
CREATE TABLE referral (
  id SERIAL NOT NULL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  user_id uuid NOT NULL REFERENCES users(id),
  friend_id uuid NOT NULL REFERENCES users(id)
);
CREATE TABLE documents (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id uuid NOT NULL REFERENCES users(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  name VARCHAR NOT NULL,
  doc_type VARCHAR NOT NULL
);
ALTER TABLE prompts ADD COLUMN document_id uuid REFERENCES documents(id);