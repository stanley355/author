use reqwest::{header::HeaderMap, Response};
use serde::Serialize;
use std::env;

use crate::{
    openai::{req::OpenAiChatReq, res::OpenAiChatRes},
    prompt::req::NewTextToSpeechPromptReq,
};

#[derive(Debug, Clone)]
pub struct OpenAi;

impl OpenAi {
    pub async fn new_chat_completion(
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<OpenAiChatRes, reqwest::Error> {
        let req_body = OpenAiChatReq::new(system_prompt, user_prompt);

        let openai_url = &env::var("OPENAI_URL").unwrap();
        let url = format!("{}v1/chat/completions", openai_url);

        let openai_key = &env::var("OPENAI_API_KEY").unwrap();
        let authorization_key = format!("Bearer {}", openai_key);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", authorization_key.parse().unwrap());

        let client = reqwest::Client::new();

        let openai_res = client
            .post(url)
            .headers(headers)
            .json(&req_body)
            .send()
            .await;

        match openai_res {
            Ok(response) => response.json::<OpenAiChatRes>().await,
            Err(err) => Err(err),
        }
    }

    pub async fn new_text_to_speech(
        req_body: OpenAiTextToSpeechReq,
    ) -> Result<actix_web::web::Bytes, reqwest::Error> {

        let openai_url = &env::var("OPENAI_URL").unwrap();
        let url = format!("{}v1/audio/speech", openai_url);

        let openai_key = &env::var("OPENAI_API_KEY").unwrap();
        let authorization_key = format!("Bearer {}", openai_key);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", authorization_key.parse().unwrap());

        let client = reqwest::Client::new();

        let openai_res = client
            .post(url)
            .headers(headers)
            .json(&req_body)
            .send()
            .await;

        match openai_res {
            Ok(response) => response.bytes().await,
            Err(err) => Err(err),
        }
    }

    
}

#[derive(Debug, Clone, Serialize)]
pub struct OpenAiTextToSpeechReq {
    model: String,
    input: String,
    voice: String,
}

impl OpenAiTextToSpeechReq {
    pub fn new(req: &NewTextToSpeechPromptReq) -> Self {
        OpenAiTextToSpeechReq {
            model: "tts-1".to_string(),
            input: req.user_prompt.clone(),
            voice: req.voice.to_string().to_lowercase(),
        }
    }
}
