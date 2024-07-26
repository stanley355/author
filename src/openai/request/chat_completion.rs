use crate::prompts::NewPromptRequest;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OpenAiChatCompletionRequest {
    model: String,
    messages: Vec<OpenAiChatCompletionRequestMessageParam>,
    n: Option<u32>,
    temperature: Option<f32>,
}

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
}

#[derive(Debug, Serialize)]
struct OpenAiChatCompletionRequestMessageParam {
    role: String,
    content: String,
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
