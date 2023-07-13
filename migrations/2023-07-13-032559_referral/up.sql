-- Your SQL goes here
CREATE TABLE referral (
  id SERIAL NOT NULL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  user_id uuid NOT NULL REFERENCES users(id),
  friend_id uuid NOT NULL REFERENCES users(id)
);