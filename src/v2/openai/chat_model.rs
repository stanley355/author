use actix_web::web;
use serde::{Deserialize, Serialize};

use crate::v2::prompt::request::NewPromptRequestBody;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiChatMessage {
    role: String,
    pub content: String,
}

impl OpenAiChatMessage {
    pub fn new_vec(system_content: &str, user_content: &str) -> Vec<Self> {
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

#[derive(Debug, Serialize)]
pub struct OpenAiChat {
    model: String,
    messages: Vec<OpenAiChatMessage>,
    n: Option<u32>,
    temperature: Option<f32>
}

impl OpenAiChat {
    pub fn new(body: &web::Json<NewPromptRequestBody>) -> Self {
        let messages = OpenAiChatMessage::new_vec(&body.system_content, &body.user_content);
        Self {
            model: "gpt-3.5-turbo".to_string(),
            messages,
            n: body.n,
            temperature: body.temperature
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenAiChatResponse {
    pub id: String,
    pub object: String,
    pub model: String,
    pub created: u32,
    pub choices: Vec<OpenAiChatChoice>,
    pub usage: OpenAiChatUsage,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiChatChoice {
    pub index: i32,
    pub finish_reason: String,
    pub message: OpenAiChatMessage,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiChatUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
