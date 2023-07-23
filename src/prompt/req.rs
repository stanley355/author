use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct NewPromptReq {
    pub user_id: String,
    pub instruction: String,
    pub prompt_token: i32,
    pub completion_token: i32,
    pub prompt_text: String,
    pub completion_text: String,
    pub document_id: Option<String>
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct FindPromptReq {
    pub doc_id: Option<String>
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct UpdatePromptReq {
    pub prompt_id: i32,
    pub instruction: String,
    pub prompt_token: i32,
    pub completion_token: i32,
    pub prompt_text: String,
    pub completion_text: String,
}