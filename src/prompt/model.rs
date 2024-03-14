use super::{req::NewPromptReq, res::NewPromptRes};
use crate::schema::prompts;
use crate::{
    db::PgPool,
    openai::{model::OpenAi, res::OpenAiChatRes},
};

use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

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
    pub async fn new(
        pool: &web::Data<PgPool>,
        body: web::Json<NewPromptReq>,
    ) -> Result<NewPromptRes, reqwest::Error> {
        let openai_result =
            OpenAi::new_chat_completion(&body.system_prompt, &body.user_prompt).await;

        match openai_result {
            Ok(result) => {
                let _prompt_save_res = Self::save_prompt(pool, &body, &result);
                let new_prompt_res =
                    NewPromptRes::new(body.into_inner(), result.choices[0].message.content.clone());
                return Ok(new_prompt_res);
            }
            Err(err) => Err(err),
        }
    }

    pub fn save_prompt(
        pool: &web::Data<PgPool>,
        new_prompt_req: &web::Json<NewPromptReq>,
        openai_chat_res: &OpenAiChatRes,
    ) -> QueryResult<Prompt> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&new_prompt_req.user_id).unwrap();
        let prompt_text = format!(
            "{} {}",
            &new_prompt_req.system_prompt, &new_prompt_req.user_prompt
        );

        let data = (
            (prompts::user_id.eq(uuid)),
            (prompts::instruction.eq(&new_prompt_req.system_prompt)),
            (prompts::prompt_token.eq(openai_chat_res.usage.prompt_tokens as i32)),
            (prompts::completion_token.eq(openai_chat_res.usage.completion_tokens as i32)),
            (prompts::prompt_text.eq(prompt_text)),
            (prompts::completion_text.eq(&openai_chat_res.choices[0].message.content)),
            (prompts::total_token.eq(openai_chat_res.usage.total_tokens as i32)),
        );

        diesel::insert_into(prompts::table)
            .values(data)
            .get_result(&conn)
    }
}
