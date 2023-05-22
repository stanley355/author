-- Your SQL goes here
CREATE TABLE checkbots (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id uuid NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  source_text_token INT NOT NULL,
  checkbot_text_token INT NOT NULL,
  source_text VARCHAR NOT NULL,
  checkbot_text VARCHAR NOT NULL,

  CONSTRAINT FK_checkbots_user FOREIGN KEY(user_id)
        REFERENCES users(id)
);