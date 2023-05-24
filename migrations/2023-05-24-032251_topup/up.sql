-- Your SQL goes here
CREATE TABLE balance_logs (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id uuid NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  topup_amount FLOAT NOT NULL DEFAULT 0.0,

  CONSTRAINT FK_balance_user FOREIGN KEY(user_id)
        REFERENCES users(id)
);