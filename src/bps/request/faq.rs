use serde::{Deserialize, Serialize};
use crate::openai::OpenAiChatCompletionRequestMessageParam;

#[derive(Debug, Deserialize, Serialize)]
pub struct BpsFaqRequest {
    pub messages: Vec<OpenAiChatCompletionRequestMessageParam>,
}

