use super::OpenAiRequestEndpoint;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;
use std::fmt::Debug;

pub trait OpenAiRequest {
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

    async fn request_json<S: DeserializeOwned>(
        &self,
        endpoint_type: OpenAiRequestEndpoint,
    ) -> Result<S, reqwest::Error>
    where
        Self: Debug + Serialize,
    {
        let url = Self::match_endpoint_url(&endpoint_type);
        let openai_key = env::var("OPENAI_API_KEY").unwrap();

        reqwest::Client::new()
            .post(url)
            .header("Authorization", format!("{} {}", "Bearer", openai_key))
            .json(&self)
            .send()
            .await?
            .json::<S>()
            .await
    }
}
