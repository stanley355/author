use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAiAudioTranscriptionsSegmentsParam {
    id: u32,
    seek: u32,
    start: f32,
    end: f32,
    text: String,
    tokens: Vec<u32>,
    temperature: f32,
    avg_logprob: f32,
    compression_ratio: f32,
    no_speech_prob: f32,
}

// #[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct OpenAiAudioTranscriptionsWordsParam {
    word: String,
    start: f32,
    end: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAiAudioTranscriptionsResponse {
    pub text: String,
    task: Option<String>,
    language: Option<String>,
    duration: Option<f32>,
    segments: Option<Vec<OpenAiAudioTranscriptionsSegmentsParam>>,
    words: Option<Vec<OpenAiAudioTranscriptionsWordsParam>>,
}