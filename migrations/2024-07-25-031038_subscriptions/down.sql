-- This file should undo anything in `up.sql`
ALTER TABLE subscriptions ADD COLUMN topup_id uuid REFERENCES topups(id);
ALTER TABLE subscriptions ADD COLUMN is_paylater boolean DEFAULT false;
ALTER TABLE subscriptions DROP COLUMN price;
-- ALTER TABLE students ADD COLUMN student_application_valid BOOLEAN NOT NULL DEFAULT true;
-- ALTER TABLE students ADD COLUMN student_application_invalid_reason VARCHAR;