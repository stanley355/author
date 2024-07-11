use serde::Serialize;
use actix_web::web;
use crate::v2::prompt::request::NewTextToSpeechRequestBody;

#[derive(Debug, Serialize)]
pub struct OpenAiAudioSpeech {
    pub model: String,
    pub input: String,
    pub voice: String,
    pub speed: Option<f32> // 0.25 - 4.0
}

impl OpenAiAudioSpeech {
    pub fn new(req: &web::Json<NewTextToSpeechRequestBody>) -> Self {
        Self {
            model: "tts-1".to_string(),
            input: req.input.clone(),
            voice: req.voice.to_string().to_lowercase(),
            speed: req.speed
        }
    }
}


#[derive(Debug, Serialize)]
pub struct OpenAiAudioTranscriptions {
    pub model: String,
    pub file_url: String,
    pub language: String,
    pub temperature: Option<f32>, // 0.0 - 1.0 
    pub timestamp_granularities: Option<Vec<String>>
}
