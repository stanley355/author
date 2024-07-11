use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub enum PromptType {
    Translate,
    Checkbot,
    TextToSpeech,
    Transcriptions
}

impl fmt::Display for PromptType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
pub struct NewPromptRequestBody {
    pub user_id: String,
    pub prompt_type: PromptType,
    pub system_content: String,
    pub user_content: String,
    pub n: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub enum TranscriptionsTimestampGranularity {
    Word,
    Segment
}

impl fmt::Display for TranscriptionsTimestampGranularity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
pub struct NewTranscriptionsRequestBody {
    pub user_id: String,
    pub file_url: String,
    pub language: Option<String>,
    pub temperature: Option<f32>,
    pub timestamp_granularities: Option<Vec<TranscriptionsTimestampGranularity>>
}