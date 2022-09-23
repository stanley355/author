-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  fullname VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR,
  phone_number VARCHAR
);