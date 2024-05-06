use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
pub enum PromptType {
    Translate,
    GrammarCheck,
    ImageToText
}

impl fmt::Display for PromptType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewPromptRequestBody {
    pub user_id: String,
    pub prompt_type: PromptType,
    pub system_content: String,
    pub user_content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateImageToTextRequestBody {
    pub user_id: String,
    pub prompt_id: i32,
    pub completion_text: String
}
