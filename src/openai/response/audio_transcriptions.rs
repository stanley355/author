use serde::{Deserialize, Serialize};

// #[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAiAudioTranscriptionsSegmentsParam {
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

// #[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAiAudioTranscriptionsWordsParam {
    pub word: String,
    pub start: f32,
    pub end: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAiAudioTranscriptionsResponse {
    pub text: String,
    task: Option<String>,
    language: Option<String>,
    pub duration: Option<f32>,
    pub segments: Option<Vec<OpenAiAudioTranscriptionsSegmentsParam>>,
    pub words: Option<Vec<OpenAiAudioTranscriptionsWordsParam>>,
}