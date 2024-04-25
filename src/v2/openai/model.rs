use reqwest::header::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};
use std::{env, fmt::Debug};

#[derive(Debug, Clone)]
pub enum OpenAiEndpointType {
    ChatCompletion,
}

pub struct OpenAi<D: Serialize> {
    base_api_url: String,
    endpoint_path: String,
    authorization_header: String,
    endpoint_type: OpenAiEndpointType,
    data: D,
}

impl<D: Serialize> OpenAi<D> {
    pub fn new(endpoint_type: OpenAiEndpointType, data: D) -> Self {
        let openai_url = env::var("OPENAI_URL").expect("Missing OpenAi Url");
        let openai_key = &env::var("OPENAI_API_KEY").unwrap();
        let authorization_header = format!("Bearer {}", openai_key);

        Self {
            base_api_url: openai_url,
            endpoint_path: Self::match_endpoint_path(&endpoint_type),
            authorization_header,
            endpoint_type,
            data,
        }
    }

    pub fn match_endpoint_path(endpoint_type: &OpenAiEndpointType) -> String {
        match endpoint_type {
            OpenAiEndpointType::ChatCompletion => "v1/chat/completions".to_string(),
        }
    }

    pub async fn request<B: DeserializeOwned>(self) -> Result<B, reqwest::Error> {
        let url = format!("{}{}", self.base_api_url, self.endpoint_path);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", self.authorization_header.parse().unwrap());

        let client = reqwest::Client::new();

        let openai_res = client
            .post(url)
            .headers(headers)
            .json(&self.data)
            .send()
            .await;

        match openai_res {
            Ok(response) => response.json::<B>().await,
            Err(err) => Err(err),
        }
    }
}
