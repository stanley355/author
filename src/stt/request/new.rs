use serde::Deserialize;
use crate::openai::OpenAiAudioTranscriptionsTimestampGranularity;

#[derive(Debug, Deserialize)]
pub struct NewSpeechToTextRequest {
    pub user_id: String,
    pub file_name: String,
    pub file_url: String,
    pub temperature: f32,
    pub language: String,
    pub timestamp_granularities: Option<OpenAiAudioTranscriptionsTimestampGranularity>
}
