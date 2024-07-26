use crate::openai::request::OpenAiChatCompletionRequestMessageParam;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenAiChatCompletionResponse {
    id: String,
    object: String,
    model: String,
    created: u32,
    pub choices: Vec<OpenAiChatCompletionResponseChoiceParameter>,
    pub usage: OpenAiChatCompletionResponseUsageParameter,
}

// #[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct OpenAiChatCompletionResponseChoiceParameter {
    index: i32,
    finish_reason: String,
    pub message: OpenAiChatCompletionRequestMessageParam,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiChatCompletionResponseUsageParameter {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
