use serde::Deserialize;
use super::PromptType;

#[derive(Debug, Deserialize)]
pub struct NewPromptRequest {
    pub user_id: String,
    pub prompt_type: PromptType,
    pub system_content: String,
    pub user_content: String,
    pub n: Option<u32>,
    pub temperature: Option<f32>,
}