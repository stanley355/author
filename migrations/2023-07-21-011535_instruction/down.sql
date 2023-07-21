-- This file should undo anything in `up.sql`
ALTER TABLE prompts DROP COLUMN instruction;
ALTER TABLE prompts DROP COLUMN document_id;