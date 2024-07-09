use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
pub enum PromptType {
    Translate,
    Checkbot,
    TextToSpeech,
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
    pub n: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum TextToSpeechVoice {
    Alloy,
    Echo,
    Fable,
    Onyx,
    Nova,
    Shimmer,
}

impl fmt::Display for TextToSpeechVoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewTextToSpeechRequestBody {
    pub user_id: String,
    pub input: String,
    pub voice: TextToSpeechVoice,
    pub speed: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTtsFileQuery {
    pub prompt_id: i32,
}
