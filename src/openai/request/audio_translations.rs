use reqwest::multipart::{Form, Part};
use serde::Serialize;

use super::OpenAiRequest;
use crate::prompts::NewAudioTranslationsRequest;

#[derive(Debug, Serialize)]
pub struct OpenAiAudioTranslationsRequest;

impl OpenAiRequest for OpenAiAudioTranslationsRequest {}

impl OpenAiAudioTranslationsRequest {
    pub async fn new_form_data(
        request: &NewAudioTranslationsRequest,
    ) -> Result<Form, reqwest::Error> {
        let file = reqwest::get(request.file_url.clone()).await?;

        let bytes = file.bytes().await?;
        let part = Part::bytes(bytes.to_vec()).file_name(request.file_name.clone());
        let form_data = Form::new()
            .part("file", part)
            .text("model", "whisper-1")
            .text("temperature", request.temperature.to_string());

        Ok(form_data)
    }
}
