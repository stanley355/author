use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone)]
pub enum OpenAiEndpointType {
    Chat,
}

#[derive(Debug, Clone)]
pub struct OpenAi<D> {
    base_api_url: String,
    endpoint_path: String,
    authorization_header: String,
    endpoint_type: OpenAiEndpointType,
    data: D,
}

impl<D> OpenAi<D> {
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
            OpenAiEndpointType::Chat => "v1/chat/completions".to_string(),
        }
    }
}
