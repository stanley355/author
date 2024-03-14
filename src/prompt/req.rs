use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewPromptReq {
    pub user_id: String,
    pub system_prompt: String,
    pub user_prompt: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenAiChatReq {
    pub model: String,
    pub messages: Vec<OpenAiChatReqMessage>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenAiChatReqMessage {
    pub role: String,
    pub content: String,
}

impl OpenAiChatReq {
    pub fn new(system_content: String, user_content: String) -> Self {
        let mut message_vec: Vec<OpenAiChatReqMessage> = Vec::new();
        message_vec.push(OpenAiChatReqMessage { role: "system".to_string(), content: system_content });
        message_vec.push(OpenAiChatReqMessage { role: "user".to_string(), content: user_content});
        OpenAiChatReq { model: "gpt-3.5-turbo-16k".to_string(), messages: message_vec }
    }
}
