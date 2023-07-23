use super::req::{NewPromptReq, UpdatePromptReq};
use crate::{db::PgPool, schema::prompts};
use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl, QueryDsl};
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
    pub document_id: Option<uuid::Uuid>,
}

impl Prompt {
    pub fn new(pool: &web::Data<PgPool>, body: web::Json<NewPromptReq>) -> QueryResult<Prompt> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
        let total_token = &body.prompt_token + &body.completion_token;

        let doc_id = match body.document_id.clone() {
            Some(id) => Some(uuid::Uuid::parse_str(&id).unwrap()),
            None => None,
        };

        let data = (
            (prompts::user_id.eq(uuid)),
            (prompts::instruction.eq(&body.instruction)),
            (prompts::prompt_token.eq(&body.prompt_token)),
            (prompts::completion_token.eq(&body.completion_token)),
            (prompts::prompt_text.eq(&body.prompt_text)),
            (prompts::completion_text.eq(&body.completion_text)),
            (prompts::total_token.eq(&total_token)),
            (prompts::document_id.eq(&doc_id)),
        );

        diesel::insert_into(prompts::table)
            .values(data)
            .get_result(&conn)
    }

    pub fn new_premium(
        pool: &web::Data<PgPool>,
        body: &web::Json<NewPromptReq>,
    ) -> QueryResult<Prompt> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
        let total_token = &body.prompt_token + &body.completion_token;

        let doc_id = match body.document_id.clone() {
            Some(id) => Some(uuid::Uuid::parse_str(&id).unwrap()),
            None => None,
        };

        let data = (
            (prompts::user_id.eq(uuid)),
            (prompts::instruction.eq(&body.instruction)),
            (prompts::prompt_token.eq(&body.prompt_token)),
            (prompts::completion_token.eq(&body.completion_token)),
            (prompts::prompt_text.eq(&body.prompt_text)),
            (prompts::completion_text.eq(&body.completion_text)),
            (prompts::total_token.eq(&total_token)),
            (prompts::total_cost.eq(total_token as f64)),
            (prompts::document_id.eq(&doc_id)),
        );

        diesel::insert_into(prompts::table)
            .values(data)
            .get_result(&conn)
    }

    pub fn find_by_doc_id(
        pool: &web::Data<PgPool>,
        doc_id: &String,
    ) -> QueryResult<Vec<Prompt>> {
        let conn = pool.get().unwrap();
        let doc_uuid = uuid::Uuid::parse_str(&doc_id).unwrap();
        prompts::table
            .filter(prompts::document_id.eq(doc_uuid))
            .get_results::<Prompt>(&conn)
    }

    pub fn update_prompt(
        pool: &web::Data<PgPool>,
        body: &web::Json<UpdatePromptReq>,
    ) -> QueryResult<Prompt> {
        let conn = pool.get().unwrap();

        let data = (
            (prompts::instruction.eq(&body.instruction)),
            (prompts::prompt_token.eq(&body.prompt_token)),
            (prompts::completion_token.eq(&body.completion_token)),
            (prompts::prompt_text.eq(&body.prompt_text)),
            (prompts::completion_text.eq(&body.completion_text)),
        );

        diesel::update(prompts::table)
            .filter(prompts::id.eq(&body.prompt_id))
            .set(data)
            .get_result::<Prompt>(&conn)
    }

    pub fn delete(pool: &web::Data<PgPool>, id: &i32) -> QueryResult<Prompt> {
        let conn = pool.get().unwrap();

        diesel::delete(prompts::table)
            .filter(prompts::id.eq(id))
            .get_result::<Prompt>(&conn)
    }
}
