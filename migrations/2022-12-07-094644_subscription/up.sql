-- Your SQL goes here
ALTER TABLE subscriptions
  DROP COLUMN channels_slug,
  DROP COLUMN channels_name,
  DROP COLUMN status;