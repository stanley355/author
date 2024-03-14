use serde::{Deserialize, Serialize};

use super::req::NewPromptReq;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewPromptRes {
    pub user_id: String,
    pub system_prompt: String,
    pub user_prompt: String,
    pub completion_text: String,
}

impl NewPromptRes {
    pub fn new(req: NewPromptReq, completion_text: String) -> Self {
        NewPromptRes {
            user_id: req.user_id,
            system_prompt: req.system_prompt,
            user_prompt: req.user_prompt,
            completion_text,
        }
    }
}
