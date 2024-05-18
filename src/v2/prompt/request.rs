use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
pub enum PromptType {
    Translate,
    GrammarCheck,
    ImageToText,
    TextToSpeech
}

impl fmt::Display for PromptType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewPromptRequestBody {
    pub user_id: String,
    pub prompt_type: PromptType,
    pub system_content: String,
    pub user_content: String,
    pub n: Option<u32>
}

#[derive(Debug, Deserialize)]
pub struct UpdateImageToTextRequestBody {
    pub user_id: String,
    pub prompt_id: i32,
    pub completion_text: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTtsFileQuery{
    pub prompt_id: i32,
}

