use diesel::{Queryable, QueryResult, RunQueryDsl, ExpressionMethods};
use serde::Serialize;
use actix_web::web;
use crate::checkbots::NewCheckbotRequest;
use crate::db::PgPool;
use crate::openai::OpenAiChatCompletionResponse;
use crate::schema::{checkbots};

#[derive(Queryable, Debug, Clone, Serialize)]
pub(super) struct Checkbots {
    id: i32,
    user_id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    instruction: String,
    model: String,
    system_content: String,
    user_content: String,
    completion_content: String,
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

impl Checkbots {
    pub(super) fn insert(pool: &web::Data<PgPool>, request: &NewCheckbotRequest, completion: &OpenAiChatCompletionResponse) -> QueryResult<Vec<Checkbots>> {
        let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();

        let data: Vec<_> = completion.choices
            .iter()
            .enumerate()
            .map(|(index, chat_choice)| {
                if index == 0 {
                    return (
                        (checkbots::user_id.eq(user_id)),
                        (checkbots::instruction.eq(&request.instruction)),
                        (checkbots::model.eq(&completion.model)),
                        (checkbots::system_content.eq(&request.system_content)),
                        (checkbots::user_content.eq(&request.user_content)),
                        (checkbots::completion_content.eq(&chat_choice.message.content)),
                        (checkbots::prompt_tokens.eq(completion.usage.prompt_tokens as i32)),
                        (checkbots::completion_tokens.eq(completion.usage.completion_tokens as i32)),
                        (checkbots::total_tokens.eq(completion.usage.total_tokens as i32))
                    );
                }
                return (
                    (checkbots::user_id.eq(user_id)),
                    (checkbots::instruction.eq(&request.system_content)),
                    (checkbots::model.eq(&completion.model)),
                    (checkbots::system_content.eq(&request.system_content)),
                    (checkbots::user_content.eq(&request.user_content)),
                    (checkbots::completion_content.eq(&chat_choice.message.content)),
                    (checkbots::prompt_tokens.eq(0)),
                    (checkbots::completion_tokens.eq(0)),
                    (checkbots::total_tokens.eq(0))
                );
            })
            .collect();

        let mut conn = pool.get().unwrap();
        diesel::insert_into(checkbots::table)
            .values(data)
            .get_results(&mut conn)
    }
}