use crate::v2::prompt::request::{NewTextToSpeechRequestBody, NewTranscriptionsRequestBody};
use actix_web::web;
use reqwest::multipart::{Form, Part};
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
    pub language: String,
    pub temperature: f32, // 0.0 - 1.0
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

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAiAudioTranscriptionsSegmentsResponse {
    pub id: u32,
    seek: u32,
    pub start: f32,
    pub end: f32,
    pub text: String,
    tokens: Vec<u32>,
    temperature: f32,
    avg_logprob: f32,
    compression_ratio: f32,
    no_speech_prob: f32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAiAudioTranscriptionsWordsResponse {
    pub word: String,
    pub start: f32,
    pub end: f32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAiAudioTranscriptionsResponse {
    pub text: String,
    task: Option<String>,
    language: Option<String>,
    pub duration: Option<f32>,
    pub segments: Option<Vec<OpenAiAudioTranscriptionsSegmentsResponse>>,
    pub words: Option<Vec<OpenAiAudioTranscriptionsWordsResponse>>,
}

#[derive(Debug, Serialize)]
pub struct OpenAiAudioTranslations;

impl OpenAiAudioTranslations {
    pub async fn new_multipart_form_data(
        file_url: &str,
        temperature: &f32,
    ) -> Result<Form, reqwest::Error> {
        let file = reqwest::get(file_url).await?;

        let bytes = file.bytes().await?;
        let part = Part::bytes(bytes.to_vec()).file_name("file.mp3");
        let form_data = Form::new()
            .part("file", part)
            .text("model", "whisper-1")
            .text("temperature", temperature.to_string());

        Ok(form_data)
    }
}
