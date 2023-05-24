-- Your SQL goes here
CREATE TABLE balance_logs (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id uuid NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  prev_balance FLOAT NOT NULL,
  increase_amount FLOAT NOT NULL DEFAULT 0.0,
  decrease_amount FLOAT NOT NULL DEFAULT 0.0,
  final_balance FLOAT NOT NULL,

  CONSTRAINT FK_balance_user FOREIGN KEY(user_id)
        REFERENCES users(id)
);