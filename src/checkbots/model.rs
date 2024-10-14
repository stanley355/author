use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Debug, Clone, Serialize)]
pub(super) struct Checkbots {
    id: i32,
    user_id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    instruction: String,
    model: String,
    system_content: String,
    user_content: String,
    pub message_content: String,
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32
}

// impl Checkbots {
//
// }