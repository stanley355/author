-- Your SQL goes here
CREATE TABLE students (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id uuid NOT NULL REFERENCES users(id),
  student_id VARCHAR NOT NULL,
  student_email VARCHAR,
  student_card_img_url VARCHAR,
  institution_level VARCHAR NOT NULL,
  institution_name VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), 
  free_discount_end_at TIMESTAMP NOT NULL,
  half_discount_end_at TIMESTAMP NOT NULL,
  student_application_valid BOOLEAN NOT NULL DEFAULT true,
  student_application_invalid_reason VARCHAR
);

SELECT diesel_manage_updated_at('students');