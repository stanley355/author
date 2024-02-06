-- This file should undo anything in `up.sql`

ALTER TABLE documents RENAME COLUMN checkbot_completion TO ai_completion;