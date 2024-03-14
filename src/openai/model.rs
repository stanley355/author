use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::env;

use crate::openai::{req::OpenAiChatReq, res::OpenAiChatRes};

#[derive(Debug, Clone)]
pub struct OpenAi;

impl OpenAi {
    pub async fn new_chat_completion(system_prompt: &str, user_prompt: &str) {
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
          .await
          .unwrap();

      println!("{:?}", openai_res.json::<OpenAiChatRes>().await);
    }
}