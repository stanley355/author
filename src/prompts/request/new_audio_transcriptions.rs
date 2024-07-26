use serde::Deserialize;
use crate::openai::OpenAiAudioTranscriptionsTimestampGranularity;

#[derive(Debug, Deserialize, Clone)]
pub struct NewAudioTranscriptionsRequest {
    pub user_id: String,
    pub file_url: String,
    pub language: String,
    pub temperature: f32,
    pub timestamp_granularities: Option<OpenAiAudioTranscriptionsTimestampGranularity>
}