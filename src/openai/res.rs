use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpenAiChatRes {
  pub id: String,
  pub object: String,
  pub model: String,
  pub created: u32,
  pub choices: Vec<OpenAiChatResChoices>,
  pub usage: OpenAiChatResUsage
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpenAiChatResChoices {
  pub index: i32,
  pub finish_reason: String,
  pub message: OpenAiChatResChoicesMsg
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpenAiChatResChoicesMsg {
  pub role: String,
  pub content: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpenAiChatResUsage {
  pub prompt_tokens: u32,
  pub completion_tokens:u32,
  pub total_tokens: u32 
}

