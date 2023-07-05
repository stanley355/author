-- This file should undo anything in `up.sql`
ALTER TABLE prompts DROP COLUMN instruction;
ALTER TABLE prompts DROP COLUMN instruction_type;
ALTER TABLE prompts DROP COLUMN original_text;
ALTER TABLE prompts DROP COLUMN is_save;