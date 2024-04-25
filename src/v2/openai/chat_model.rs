use serde::{Deserialize, Serialize};

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
}

impl OpenAiChat {
    pub fn new(system_content: &str, user_content: &str) -> Self {
        let messages = OpenAiChatMessage::new_vec(system_content, user_content);
        Self {
            model: "gpt-3.5-turbo-16k".to_string(),
            messages,
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
