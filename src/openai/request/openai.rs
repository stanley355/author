use super::OpenAiRequestEndpoint;
use reqwest::multipart::Form;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;
use std::fmt::Debug;

#[derive(Debug)]
pub struct OpenAiRequest<D: Debug + Serialize> {
    endpoint_type: OpenAiRequestEndpoint,
    data: Option<D>,
    form_data: Option<Form>,
}

impl<D: Debug + Serialize> OpenAiRequest<D> {
    pub fn new(
        endpoint_type: OpenAiRequestEndpoint,
        data: Option<D>,
        form_data: Option<Form>,
    ) -> Self {
        Self {
            endpoint_type,
            data,
            form_data,
        }
    }

    fn match_endpoint_url(endpoint: &OpenAiRequestEndpoint) -> String {
        let openai_url = env::var("OPENAI_URL").expect("Missing OPENAI_URL");
        let path = match endpoint {
            OpenAiRequestEndpoint::ChatCompletion => "v1/chat/completions".to_string(),
            OpenAiRequestEndpoint::AudioSpeech => "v1/audio/speech".to_string(),
            OpenAiRequestEndpoint::AudioTranscriptions => "v1/audio/transcriptions".to_string(),
            OpenAiRequestEndpoint::AudioTranslations => "v1/audio/translations".to_string(),
        };

        format!("{}{}", openai_url, path)
    }

    pub async fn request_json<S: DeserializeOwned>(self) -> Result<S, reqwest::Error> {
        let url = Self::match_endpoint_url(&self.endpoint_type);
        let openai_key = env::var("OPENAI_API_KEY").unwrap();

        reqwest::Client::new()
            .post(url)
            .header("Authorization", format!("{} {}", "Bearer", openai_key))
            .json(&self.data)
            .send()
            .await?
            .json::<S>()
            .await
    }
}
