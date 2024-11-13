use crate::openai::request::OpenAiChatCompletionRequestMessageParam;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OpenAiChatCompletionResponse {
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    object: String,
    pub model: String,
    #[allow(dead_code)]
    created: u32,
    pub choices: Vec<OpenAiChatCompletionResponseChoiceParameter>,
    pub usage: OpenAiChatCompletionResponseUsageParameter,
}

// #[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAiChatCompletionResponseChoiceParameter {
    #[allow(dead_code)]
    index: i32,
    #[allow(dead_code)]
    finish_reason: String,
    pub message: OpenAiChatCompletionRequestMessageParam,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiChatCompletionResponseUsageParameter {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
