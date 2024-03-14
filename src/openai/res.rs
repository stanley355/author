use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpenAiChatRes {
  pub id: String,
  pub object: String,
  pub model: String,
  pub created: u32,
  pub choices: Vec<OpenAiChatResChoices>
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
