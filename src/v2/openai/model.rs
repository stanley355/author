use super::audio_model::OpenAiAudioTranscriptions;
use actix_web::web;
use reqwest::{
    header::HeaderMap,
    multipart::{Form, Part},
};
use serde::{de::DeserializeOwned, Serialize};
use std::{env, fmt::Debug};
use futures_util::StreamExt;

#[derive(Debug, Clone)]
pub enum OpenAiEndpointType {
    ChatCompletion,
    AudioSpeech,
    AudioTranscriptions,
}

pub struct OpenAi<D: Serialize> {
    base_api_url: String,
    endpoint_path: String,
    authorization_header: String,
    #[allow(dead_code)]
    pub endpoint_type: OpenAiEndpointType,
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
            OpenAiEndpointType::AudioSpeech => "v1/audio/speech".to_string(),
            OpenAiEndpointType::AudioTranscriptions => "v1/audio/transcriptions".to_string(),
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

    pub async fn request_bytes(self) -> Result<web::Bytes, reqwest::Error> {
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
            Ok(response) => response.bytes().await,
            Err(err) => Err(err),
        }
    }

    pub async fn request_bytes_stream(self) -> Result<(), reqwest::Error> {
        let url = format!("{}{}", self.base_api_url, self.endpoint_path);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", self.authorization_header.parse().unwrap());

        let client = reqwest::Client::new();

        let mut openai_stream = client
            .post(url)
            .headers(headers)
            .json(&self.data)
            .send()
            .await?.bytes_stream();


            while let Some(item) = openai_stream.next().await {
                println!("Chunk: {:?}", item?);
            }
            Ok(())
        // match openai_res {
        //     Ok(response) => response.json::<B>().await,
        //     Err(err) => Err(err),
        // }
    }

    pub async fn request_transcriptions<B: DeserializeOwned + Debug>(
        self,
        req: &OpenAiAudioTranscriptions,
    ) -> Result<B, reqwest::Error> {
        let file_result = reqwest::get(&req.file_url).await;

        match file_result {
            Ok(file) => {
                let bytes = file.bytes().await?;
                let part = Part::bytes(bytes.to_vec()).file_name("file.mp3");
                let mut form = Form::new()
                    .part("file", part)
                    .text("model", req.model.clone())
                    .text("language", req.language.clone())
                    .text("temperature", req.temperature.clone().to_string());

                if let Some(granularity) = req.timestamp_granularities.clone() {
                    let new_part = Part::bytes(bytes.to_vec()).file_name("file.mp3");
                    form = Form::new()
                        .part("file", new_part)
                        .text("model", req.model.clone())
                        .text("language", req.language.clone())
                        .text("temperature", req.temperature.clone().to_string())
                        .text("response_format", "verbose_json")
                        .text("timestamp_granularities[]", granularity);
                }

                let client = reqwest::Client::new();
                let url = format!("{}{}", self.base_api_url, self.endpoint_path);
                let mut headers = HeaderMap::new();
                headers.insert("Authorization", self.authorization_header.parse().unwrap());

                let openai_resp = client
                    .post(url)
                    .headers(headers)
                    .multipart(form)
                    .send()
                    .await;

                return match openai_resp {
                    Ok(response) => response.json::<B>().await,
                    Err(err) => Err(err),
                };
            }
            Err(req_file_err) => Err(req_file_err),
        }
    }
}
