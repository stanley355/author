-- Your SQL goes here
CREATE TABLE documents (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id uuid NOT NULL REFERENCES users(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), 
  name VARCHAR NOT NULL,
  content TEXT,
  ai_completion TEXT
);

SELECT diesel_manage_updated_at('documents');