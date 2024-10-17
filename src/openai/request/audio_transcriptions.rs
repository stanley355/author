use reqwest::multipart::{Form, Part};
use serde::Serialize;

use crate::prompts::NewAudioTranscriptionsRequest;
// use crate::stt::NewSpeechToTextRequest;
use super::OpenAiRequest;

#[derive(Debug, Serialize)]
pub struct OpenAiAudioTranscriptionsRequest;

impl OpenAiRequest for OpenAiAudioTranscriptionsRequest {}

impl OpenAiAudioTranscriptionsRequest {
    pub async fn new_form_data(
        req: &NewAudioTranscriptionsRequest,
    ) -> Result<Form, reqwest::Error> {
        let file = reqwest::get(req.file_url.clone()).await?;

        let bytes = file.bytes().await?;
        let part = Part::bytes(bytes.to_vec()).file_name(req.file_name.clone());

        let form_data = match &req.timestamp_granularities {
            Some(granularity) => Form::new()
                .part("file", part)
                .text("model", "whisper-1")
                .text("language", req.language.clone())
                .text("temperature", req.temperature.to_string())
                .text("response_format", "verbose_json")
                .text(
                    "timestamp_granularities[]",
                    granularity.to_string().to_lowercase(),
                ),
            None => Form::new()
                .part("file", part)
                .text("model", "whisper-1")
                .text("language", req.language.clone())
                .text("temperature", req.temperature.to_string()),
        };

        Ok(form_data)
    }

    pub async fn new(
        req: &NewSpeechToTextRequest,
    ) -> Result<Form, reqwest::Error> {
        let file = reqwest::get(req.file_url.clone()).await?;

        let bytes = file.bytes().await?;
        let part = Part::bytes(bytes.to_vec()).file_name(req.file_name.clone());

        let form_data = match &req.timestamp_granularities {
            Some(granularity) => Form::new()
                .part("file", part)
                .text("model", "whisper-1")
                .text("language", req.language.clone())
                .text("temperature", req.temperature.to_string())
                .text("response_format", "verbose_json")
                .text(
                    "timestamp_granularities[]",
                    granularity.to_string().to_lowercase(),
                ),
            None => Form::new()
                .part("file", part)
                .text("model", "whisper-1")
                .text("language", req.language.clone())
                .text("temperature", req.temperature.to_string()),
        };

        Ok(form_data)
    }
}
