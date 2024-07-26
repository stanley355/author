use actix_web::web;
use diesel::{
    dsl, BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;

use super::payment::PromptPayment;
use super::request::PromptType;
use super::{NewAudioSpeechPromptRequest, NewPromptRequest};
use crate::db::PgPool;
use crate::openai::OpenAiChatCompletionResponse;
use crate::schema::prompts;
use crate::students::Student;
use crate::subscriptions::Subscription;

#[derive(Queryable, Debug, Clone, Serialize)]
pub(super) struct Prompt {
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
    pub prompt_type: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Prompt {
    pub(super) fn check_payment(
        pool: &web::Data<PgPool>,
        user_id: &uuid::Uuid,
        prompt_type: &PromptType,
    ) -> PromptPayment {
        if let Ok(_) = Student::find_user_last_active_application(pool, user_id) {
            return PromptPayment::Student;
        }

        if let Ok(_) = Subscription::find_active(pool, user_id) {
            return PromptPayment::Subscription;
        }

        if let Ok(count) = Self::count_user_monthly_prompt(pool, user_id, prompt_type) {
            if count <= 5 {
                return PromptPayment::MonthlyQuota;
            }
        }

        PromptPayment::PaymentRequired
    }

    pub fn count_user_monthly_prompt(
        pool: &web::Data<PgPool>,
        user_id: &uuid::Uuid,
        prompt_type: &PromptType,
    ) -> QueryResult<i64> {
        let mut conn = pool.get().unwrap();

        prompts::table
            .filter(
                prompts::user_id
                    .eq(user_id)
                    .and(prompts::prompt_type.eq(prompt_type.to_string()))
                    .and(
                        prompts::created_at
                            .between(dsl::sql("date_trunc('month', now())"), dsl::sql("now()")),
                    ),
            )
            .count()
            .get_result(&mut conn)
    }

    pub fn new_insert_chat_completion(
        pool: &web::Data<PgPool>,
        request: &NewPromptRequest,
        chat_completion_response: &OpenAiChatCompletionResponse,
    ) -> QueryResult<Vec<Prompt>> {
        let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();
        let prompt_type = &request.prompt_type.to_string();

        let data: Vec<_> = chat_completion_response
            .choices
            .iter()
            .enumerate()
            .map(|(index, chat_choice)| {
                if index == 0 {
                    return (
                        (prompts::user_id.eq(user_id)),
                        (prompts::instruction.eq(&request.system_content)),
                        (prompts::prompt_token
                            .eq(chat_completion_response.usage.prompt_tokens as i32)),
                        (prompts::completion_token
                            .eq(chat_completion_response.usage.completion_tokens as i32)),
                        (prompts::prompt_text.eq(&request.user_content)),
                        (prompts::completion_text.eq(&chat_choice.message.content)),
                        (prompts::total_token
                            .eq(chat_completion_response.usage.total_tokens as i32)),
                        (prompts::total_cost.eq(0.0)),
                        (prompts::prompt_type.eq(prompt_type)),
                    );
                }

                return (
                    (prompts::user_id.eq(user_id)),
                    (prompts::instruction.eq(&request.system_content)),
                    (prompts::prompt_token.eq(0)),
                    (prompts::completion_token.eq(0)),
                    (prompts::prompt_text.eq(&request.user_content)),
                    (prompts::completion_text.eq(&chat_choice.message.content)),
                    (prompts::total_token.eq(0)),
                    (prompts::total_cost.eq(0.0)),
                    (prompts::prompt_type.eq(prompt_type)),
                );
            })
            .collect();

        let mut conn = pool.get().unwrap();
        diesel::insert_into(prompts::table)
            .values(data)
            .get_results(&mut conn)
    }

    pub fn new_insert_audio_speech(
        pool: &web::Data<PgPool>,
        request: &NewAudioSpeechPromptRequest,
    ) -> QueryResult<Prompt> {
        let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();

        let data = (
            (prompts::user_id.eq(user_id)),
            (prompts::prompt_token.eq(0)),
            (prompts::completion_token.eq(0)),
            (prompts::prompt_text.eq(&request.input)),
            (prompts::completion_text.eq(&"")),
            (prompts::total_token.eq(0)),
            (prompts::total_cost.eq(0.0)),
            (prompts::instruction.eq(&"")),
            (prompts::prompt_type.eq(PromptType::AudioSpeech.to_string())),
        );

        let mut conn = pool.get().unwrap();
        diesel::insert_into(prompts::table)
            .values(data)
            .get_result(&mut conn)
    }

    pub fn new_insert_audio_transcriptions(
        pool: &web::Data<PgPool>,
        user_id: &uuid::Uuid,
        text: &str,
    ) -> QueryResult<Prompt> {
        let mut conn = pool.get().unwrap();

        let data = (
            (prompts::user_id.eq(user_id)),
            (prompts::prompt_token.eq(0)),
            (prompts::completion_token.eq(0)),
            (prompts::prompt_text.eq(&"")),
            (prompts::completion_text.eq(text)),
            (prompts::total_token.eq(0)),
            (prompts::total_cost.eq(0.0)),
            (prompts::instruction.eq(&"")),
            (prompts::prompt_type.eq(PromptType::AudioTranscriptions.to_string())),
        );

        diesel::insert_into(prompts::table)
            .values(data)
            .get_result(&mut conn)
    }

    pub fn new_insert_audio_translations(
        pool: &web::Data<PgPool>,
        user_id: &uuid::Uuid,
        text: &str,
    ) -> QueryResult<Prompt> {
        let mut conn = pool.get().unwrap();

        let data = (
            (prompts::user_id.eq(user_id)),
            (prompts::prompt_token.eq(0)),
            (prompts::completion_token.eq(0)),
            (prompts::prompt_text.eq(&"")),
            (prompts::completion_text.eq(text)),
            (prompts::total_token.eq(0)),
            (prompts::total_cost.eq(0.0)),
            (prompts::instruction.eq(&"")),
            (prompts::prompt_type.eq(PromptType::AudioTranslations.to_string())),
        );

        diesel::insert_into(prompts::table)
            .values(data)
            .get_result(&mut conn)
    }
}
