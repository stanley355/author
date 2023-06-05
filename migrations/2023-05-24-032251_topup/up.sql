-- Your SQL goes here
CREATE TABLE topups (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id uuid NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  topup_amount FLOAT NOT NULL DEFAULT 0.0,
  paid BOOLEAN NOT NULL DEFAULT false,

  CONSTRAINT FK_balance_user FOREIGN KEY(user_id)
        REFERENCES users(id)
);