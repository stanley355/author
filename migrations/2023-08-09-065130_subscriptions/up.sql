-- Your SQL goes here
CREATE TABLE subscriptions (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id uuid NOT NULL REFERENCES users(id),
  topup_id uuid NOT NULL REFERENCES topups(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  start_at TIMESTAMP NOT NULL DEFAULT NOW(),
  end_at TIMESTAMP NOT NULL,
  duration_type VARCHAR NOT NULL,
  paid BOOLEAN NOT NULL DEFAULT false
);