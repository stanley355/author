-- Your SQL goes here
CREATE TABLE checkbots (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id uuid NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  prompt_token INT NOT NULL,
  completion_token INT NOT NULL,
  prompt_text VARCHAR NOT NULL,
  completion_text VARCHAR NOT NULL,

  CONSTRAINT FK_checkbots_user FOREIGN KEY(user_id)
        REFERENCES users(id)
);