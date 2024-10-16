use diesel::{Queryable, QueryResult, RunQueryDsl, ExpressionMethods};
use serde::Serialize;
use actix_web::web;
use crate::db::PgPool;
use crate::openai::OpenAiChatCompletionResponse;
use crate::schema::{translation};
use crate::translation::request::NewTranslationRequest;

#[derive(Queryable, Debug, Clone, Serialize)]
pub(super) struct Translation {
    id: i32,
    user_id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    content_language: String,
    target_language: String,
    model: String,
    system_content: String,
    user_content: String,
    completion_content: String,
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

impl Translation {
    pub(super) fn insert(pool: &web::Data<PgPool>, request: &NewTranslationRequest, completion: &OpenAiChatCompletionResponse) -> QueryResult<Vec<Translation>> {
        let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();

        let data: Vec<_> = completion.choices
            .iter()
            .enumerate()
            .map(|(index, chat_choice)| {
                if index == 0 {
                    return (
                        (translation::user_id.eq(user_id)),
                        (translation::content_language.eq(&request.content_language)),
                        (translation::target_language.eq(&request.target_language)),
                        (translation::model.eq(&completion.model)),
                        (translation::system_content.eq(&request.system_content)),
                        (translation::user_content.eq(&request.user_content)),
                        (translation::completion_content.eq(&chat_choice.message.content)),
                        (translation::prompt_tokens.eq(completion.usage.prompt_tokens as i32)),
                        (translation::completion_tokens.eq(completion.usage.completion_tokens as i32)),
                        (translation::total_tokens.eq(completion.usage.total_tokens as i32))
                    );
                }
                return (
                    (translation::user_id.eq(user_id)),
                    (translation::content_language.eq(&request.content_language)),
                    (translation::target_language.eq(&request.target_language)),
                    (translation::model.eq(&completion.model)),
                    (translation::system_content.eq(&request.system_content)),
                    (translation::user_content.eq(&request.user_content)),
                    (translation::completion_content.eq(&chat_choice.message.content)),
                    (translation::prompt_tokens.eq(0)),
                    (translation::completion_tokens.eq(0)),
                    (translation::total_tokens.eq(0))
                );
            })
            .collect();

        let mut conn = pool.get().unwrap();
        diesel::insert_into(translation::table)
            .values(data)
            .get_results(&mut conn)
    }
}
