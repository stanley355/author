-- Your SQL goes here
CREATE TABLE subscriptions (
  id SERIAL PRIMARY KEY,
  user_id uuid NOT NULL,
  channels_id INTEGER NOT NULL,
  channels_slug VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP(0),
  expired_at TIMESTAMP,
  monthly_price INTEGER NOT NULL,
  total_price INTEGER NOT NULL,
  status VARCHAR NOT NULL DEFAULT 'Pending',
  FOREIGN KEY (user_id) REFERENCES users (id)
);