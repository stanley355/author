use diesel::{Queryable, QueryResult, RunQueryDsl, ExpressionMethods};
use serde::Serialize;
use actix_web::web;
use crate::db::PgPool;
use crate::openai::OpenAiChatCompletionResponse;
// use crate::schema::{translation};
// use crate::translation::request::NewTranslationRequest;

#[derive(Queryable, Debug, Clone, Serialize)]
pub(super) struct SpeechToText {
    id: i32,
    user_id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    model: String,
    file_name: String,
    file_url: String,
    language: String,
    timestamp_granularity: String
}
