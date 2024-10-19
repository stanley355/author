-- Your SQL goes here
CREATE TABLE text_to_speech(
  id SERIAL PRIMARY KEY NOT NULL,
  user_id uuid NOT NULL REFERENCES users(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  model VARCHAR NOT NULL,
  input VARCHAR NOT NULL,
  voice VARCHAR NOT NULL,
  speed INT NOT NULL DEFAULT 1.0,
  response_format VARCHAR NOT NULL
);
