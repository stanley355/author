-- Your SQL goes here
ALTER TABLE users
  ADD COLUMN has_channel BOOLEAN NOT NULL DEFAULT false;