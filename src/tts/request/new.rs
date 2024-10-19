use serde::Deserialize;
use crate::openai::{OpenAiAudioSpeechResponseFormat, OpenAiAudioSpeechVoice};

#[derive(Debug, Deserialize)]
pub struct NewAudioSpeechPromptRequest {
    pub user_id: String,
    pub input: String,
    pub voice: OpenAiAudioSpeechVoice,
    pub speed: f32,
    pub response_format: OpenAiAudioSpeechResponseFormat
}
