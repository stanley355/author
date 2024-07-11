use crate::v2::prompt::request::{NewTextToSpeechRequestBody, NewTranscriptionsRequestBody};
use actix_web::web;
use serde::{Deserialize, Serialize};

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
    pub timestamp_granularities: Option<String>,
}

impl OpenAiAudioTranscriptions {
    pub fn new(req: &web::Json<NewTranscriptionsRequestBody>) -> Self {
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

#[derive(Debug, Deserialize)]
pub struct OpenAiAudioTranscriptionsSegmentsResponse {
    pub id: u32,
    seek: u32,
    pub start: f32,
    pub end: f32,
    tokens: Vec<u32>,
    temperature: f32,
    avg_logprob: f32,
    compression_ratio: f32,
    no_speech_prob: f32
}

#[derive(Debug, Deserialize)]
pub struct OpenAiAudioTranscriptionsWordsResponse {
    pub word: String,
    pub start: f32,
    pub end: f32,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiAudioTranscriptionsResponse {
    pub text: String,
    task: Option<String>,
    language: Option<String>,
    pub duration: Option<f32>,
    pub segments: Option<Vec<OpenAiAudioTranscriptionsSegmentsResponse>>,
    pub words:  Option<Vec<OpenAiAudioTranscriptionsWordsResponse>>
}
