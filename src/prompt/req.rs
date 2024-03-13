use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct NewPromptReq {
    pub user_id: String,
    pub system_prompt: String,
    pub user_prompt: String
}
