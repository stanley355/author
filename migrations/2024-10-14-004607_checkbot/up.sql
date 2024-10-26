-- Your SQL goes here
CREATE TABLE checkbots (
  id SERIAL PRIMARY KEY NOT NULL,
  user_id uuid NOT NULL REFERENCES users(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  instruction VARCHAR NOT NULL,
  model VARCHAR NOT NULL,
  system_content TEXT NOT NULL,
  user_content TEXT NOT NULL,
  completion_content TEXT NOT NULL,
  prompt_tokens INT NOT NULL,
  completion_tokens INT NOT NULL,
  total_tokens INT NOT NULL
);
