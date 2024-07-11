use crate::v2::prompt::request::{NewTextToSpeechRequestBody, NewTranscriptionsRequestBody};
use actix_web::web;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OpenAiAudioSpeech {
    pub model: String,
    pub input: String,
    pub voice: String,
    pub speed: Option<f32>, // 0.25 - 4.0
}

impl OpenAiAudioSpeech {
    pub fn new(req: &web::Json<NewTextToSpeechRequestBody>) -> Self {
        Self {
            model: "tts-1".to_string(),
            input: req.input.clone(),
            voice: req.voice.to_string().to_lowercase(),
            speed: req.speed,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OpenAiAudioTranscriptions {
    pub model: String,
    pub file_url: String,
    pub language: Option<String>,
    pub temperature: Option<f32>, // 0.0 - 1.0
    pub timestamp_granularities: Option<Vec<String>>,
}

impl OpenAiAudioTranscriptions {
    pub fn new(req: &web::Json<NewTranscriptionsRequestBody>) -> Self {
        Self {
            model: "whisper-1".to_string(),
            file_url: req.file_url.clone(),
            language: req.language.clone(),
            temperature: req.temperature,
            timestamp_granularities: match req.timestamp_granularities.clone() {
                Some(granularities) => granularities
                    .iter()
                    .map(|granularity| Some(granularity.to_string().to_lowercase()))
                    .collect(),
                None => None,
            },
        }
    }
}
