use crate::prompts::NewPromptRequest;
use serde::{Deserialize, Serialize};
use crate::checkbots::NewCheckbotRequest;
use crate::translation::NewTranslationRequest;

use super::OpenAiRequest;

#[derive(Debug, Serialize)]
pub struct OpenAiChatCompletionRequest {
    model: String,
    messages: Vec<OpenAiChatCompletionRequestMessageParam>,
    n: Option<u32>,
    temperature: Option<f32>,
}

impl OpenAiRequest for OpenAiChatCompletionRequest {}
impl OpenAiChatCompletionRequest {
  pub fn new(request: &NewPromptRequest) -> Self {
    let messages = OpenAiChatCompletionRequestMessageParam::new_vec(
      &request.system_content,
      &request.user_content,
    );
    Self {
      model: "gpt-4o-mini".to_string(),
      messages,
      n: request.n,
      temperature: request.temperature,
    }
  }

    pub fn new_checkbot(request: &NewCheckbotRequest) -> Self {
        let messages = OpenAiChatCompletionRequestMessageParam::new_vec(&request.system_content, &request.user_content);
        Self {
            model: "gpt-4o-mini".to_string(),
            messages,
            n: request.n,
            temperature: request.temperature,
        }
    }
    pub fn new_translation(request: &NewTranslationRequest) -> Self {
        let messages = OpenAiChatCompletionRequestMessageParam::new_vec(&request.system_content, &request.user_content);
        Self {
            model: "gpt-4o-mini".to_string(),
            messages,
            n: request.n,
            temperature: request.temperature,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct OpenAiChatCompletionRequestMessageParam {
  role: String,
  pub(crate) content: String,
}

impl OpenAiChatCompletionRequestMessageParam {
  fn new_vec(system_content: &str, user_content: &str) -> Vec<Self> {
    let mut message_vec: Vec<Self> = Vec::new();
    message_vec.push(Self {
      role: "system".to_string(),
      content: system_content.to_string(),
    });
    message_vec.push(Self {
            role: "user".to_string(),
            content: user_content.to_string(),
        });

        message_vec
    }
}

