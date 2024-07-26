use serde::Serialize;

use crate::prompts::NewAudioSpeechPromptRequest;

use super::OpenAiRequest;

#[derive(Debug, Serialize)]
pub struct OpenAiAudioSpeech {
    pub model: String,
    pub input: String,
    pub voice: String,
    pub speed: Option<f32>, // 0.25 - 4.0
}

impl OpenAiRequest for OpenAiAudioSpeech {}

impl OpenAiAudioSpeech {
    pub fn new(req: &NewAudioSpeechPromptRequest) -> Self {
        Self {
            model: "tts-1".to_string(),
            input: req.input.clone(),
            voice: req.voice.to_string().to_lowercase(),
            speed: req.speed,
        }
    }
}
