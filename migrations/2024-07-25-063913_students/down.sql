-- This file should undo anything in `up.sql`
ALTER TABLE students ADD COLUMN student_application_valid BOOLEAN NOT NULL DEFAULT true;
ALTER TABLE students ADD COLUMN student_application_invalid_reason VARCHAR;