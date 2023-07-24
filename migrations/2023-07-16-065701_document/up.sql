-- Your SQL goes here
CREATE TABLE documents (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id uuid NOT NULL REFERENCES users(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  name VARCHAR NOT NULL,
  doc_type VARCHAR NOT NULL
);