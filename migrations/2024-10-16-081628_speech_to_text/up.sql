-- Your SQL goes here
CREATE TABLE speech_to_text (
  id SERIAL PRIMARY KEY NOT NULL,
  user_id uuid NOT NULL REFERENCES users(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  model VARCHAR NOT NULL,
  file_name VARCHAR NOT NULL,
  file_url VARCHAR NOT NULL,
  language VARCHAR,
  timestamp_granularity VARCHAR NOT NULL
);
