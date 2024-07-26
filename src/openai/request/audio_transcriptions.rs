use serde::Serialize;

use crate::prompts::NewAudioTranscriptionsRequest;

use super::OpenAiRequest;

#[derive(Debug, Serialize)]
pub struct OpenAiAudioTranscriptionsRequest {
    pub model: String,
    pub file_url: String,
    pub language: String,
    pub temperature: f32, // 0.0 - 1.0
    pub timestamp_granularities: Option<String>,
}

impl OpenAiRequest for OpenAiAudioTranscriptionsRequest {}

impl OpenAiAudioTranscriptionsRequest {
    pub fn new(req: &NewAudioTranscriptionsRequest) -> Self {
        Self {
            model: "whisper-1".to_string(),
            file_url: req.file_url.clone(),
            language: req.language.clone(),
            temperature: req.temperature,
            timestamp_granularities: match req.timestamp_granularities.clone() {
                Some(granularity) => Some(granularity.to_string().to_lowercase()),
                None => None,
            },
        }
    }
}
