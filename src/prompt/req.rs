use diesel::Queryable;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct NewPromptReq {
    pub user_id: String,
    pub prompt_token: i32,
    pub completion_token: i32,
    pub prompt_text: String,
    pub completion_text: String,
    pub instruction: String,
    pub instruction_type: InstructionType,
    pub original_text: String,
    pub is_save: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum InstructionType {
    Translate,
    Checkbot,
}

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstructionType::Translate => write!(f, "translate"),
            InstructionType::Checkbot => write!(f, "Checkbot"),
        }
    }
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct FindSavedPromptReq {
    pub user_id: String,
    pub instruction_type: InstructionType,
}
