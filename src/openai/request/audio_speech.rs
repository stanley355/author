use serde::Serialize;

use crate::tts::NewTextToSpeechRequest;
use super::OpenAiRequest;

#[derive(Debug, Serialize)]
pub struct OpenAiAudioSpeech {
    pub model: String,
    pub input: String,
    pub voice: String,
    pub speed: Option<f32>, // 0.25 - 4.0
    response_format: String
}

impl OpenAiRequest for OpenAiAudioSpeech {}

impl OpenAiAudioSpeech {
    pub fn new_text_to_speech(req: &NewTextToSpeechRequest) -> Self {
        Self {
            model: "tts-1".to_string(),
            input: req.input.clone(),
            voice: req.voice.to_string().to_lowercase(),
            speed: Some(req.speed),
            response_format: req.response_format.to_string().to_lowercase()
        }
    }
}
