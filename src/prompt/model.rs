use super::req::NewPromptReq;
use crate::{db::PgPool, document::req, prompt::req::OpenAiChatReq, schema::prompts};

use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, env};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Prompt {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub prompt_token: i32,
    pub completion_token: i32,
    pub prompt_text: String,
    pub completion_text: String,
    pub total_token: i32,
    pub total_cost: f64,
    pub instruction: String,
}

impl Prompt {
    pub async fn new(pool: &web::Data<PgPool>, body: web::Json<NewPromptReq>) {
        let req_body = OpenAiChatReq::new(body.system_prompt.clone(), body.user_prompt.clone());

        let openai_url = &env::var("OPENAI_URL").unwrap();
        let url = format!("{}v1/chat/completions", openai_url);
        let openai_key = &env::var("OPENAI_API_KEY").unwrap();
        let authorization_key = format!("Bearer {}", openai_key);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", authorization_key.parse().unwrap());
        let client = reqwest::Client::new();

        let b = client
            
            .post(url)
            .headers(headers)
            .json(&req_body)
            .send()
            .await
            .unwrap();
        // .headers(headers)
        // .json(&map)
        // .send()
        // .await?.json();
        println!("{:?}", b.text().await);
        // let conn = pool.get().unwrap();
        // let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
        // let total_token = &body.prompt_token + &body.completion_token;

        // let data = (
        //     (prompts::user_id.eq(uuid)),
        //     (prompts::instruction.eq(&body.instruction)),
        //     (prompts::prompt_token.eq(&body.prompt_token)),
        //     (prompts::completion_token.eq(&body.completion_token)),
        //     (prompts::prompt_text.eq(&body.prompt_text)),
        //     (prompts::completion_text.eq(&body.completion_text)),
        //     (prompts::total_token.eq(&total_token)),
        // );

        // diesel::insert_into(prompts::table)
        //     .values(data)
        //     .get_result(&conn)
    }
}
