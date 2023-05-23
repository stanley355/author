use actix_web::web;
use diesel::{Queryable, QueryResult, ExpressionMethods, RunQueryDsl};
use serde::{Deserialize, Serialize};
use crate::{db::PgPool, schema::prompts};
use super::req::NewPromptReq;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Prompt {
  pub id: i32,
  pub user_id: uuid::Uuid,
  pub created_at: chrono::NaiveDateTime,
  pub prompt_token: i32,
  pub completion_token: i32,
  pub prompt_text: String,
  pub completion_text: String,
}

impl Prompt {
  pub fn new(pool: &web::Data<PgPool>, body: web::Json<NewPromptReq>) -> QueryResult<Prompt> {
    let conn = pool.get().unwrap();
    let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
    let data = (
        (prompts::user_id.eq(uuid)),
        (prompts::prompt_token.eq(&body.prompt_token)),
        (prompts::completion_token.eq(&body.completion_token)),
        (prompts::prompt_text.eq(&body.prompt_text)),
        (prompts::completion_text.eq(&body.completion_text))
    );

    diesel::insert_into(prompts::table)
        .values(data)
        .get_result(&conn)
  }
}

