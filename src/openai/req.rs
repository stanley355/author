use serde::{Deserialize, Serialize};


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
    pub fn new(system_content: &str, user_content: &str) -> Self {
        let mut message_vec: Vec<OpenAiChatReqMessage> = Vec::new();
        message_vec.push(OpenAiChatReqMessage { role: "system".to_string(), content: system_content.to_string() });
        message_vec.push(OpenAiChatReqMessage { role: "user".to_string(), content: user_content.to_string()});
        OpenAiChatReq { model: "gpt-3.5-turbo-16k".to_string(), messages: message_vec }
    }
}
