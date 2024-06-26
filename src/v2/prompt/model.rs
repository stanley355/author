use super::request::{PromptType, UpdateImageToTextRequestBody};
use crate::schema::prompts;
use crate::v2::openai::audio_model::OpenAiAudioSpeech;
use crate::v2::openai::chat_model::{OpenAiChat, OpenAiChatResponse};
use crate::v2::openai::model::OpenAiEndpointType;
use crate::v2::prompt::request::NewPromptRequestBody;
use crate::{db::PgPool, v2::openai::model::OpenAi};

use actix_web::web;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;

#[derive(Queryable, Debug, Clone, Serialize)]
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
    pub prompt_type: Option<String>,
}

impl Prompt {
    pub fn count_user_monthly_count(
        pool: &web::Data<PgPool>,
        user_id: &str,
        prompt_type: &PromptType,
    ) -> QueryResult<i64> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(user_id).unwrap();

        prompts::table
            .filter(
                prompts::user_id
                    .eq(uuid)
                    .and(prompts::prompt_type.eq(prompt_type.to_string()))
                    .and(prompts::created_at.between(
                        diesel::dsl::sql("date_trunc('month', now())"),
                        diesel::dsl::sql("now()"),
                    )),
            )
            .count()
            .get_result(&mut conn)
    }

    pub fn new_instruct_insert(
        pool: &web::Data<PgPool>,
        body: &web::Json<NewPromptRequestBody>,
        openai_chat_res: OpenAiChatResponse,
    ) -> QueryResult<Vec<Prompt>> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();

        let prompt_type = &body.prompt_type.to_string();

        let data: Vec<_> = openai_chat_res
            .choices
            .iter()
            .enumerate()
            .map(|(index, chat_choice)| {
                if index > 0 {
                    return (
                        (prompts::user_id.eq(uuid)),
                        (prompts::instruction.eq(&body.system_content)),
                        (prompts::prompt_token.eq(0)),
                        (prompts::completion_token.eq(0)),
                        (prompts::prompt_text.eq(&body.user_content)),
                        (prompts::completion_text.eq(&chat_choice.message.content)),
                        (prompts::total_token.eq(0)),
                        (prompts::total_cost.eq(0.0)),
                        (prompts::prompt_type.eq(prompt_type)),
                    );
                }

                return (
                    (prompts::user_id.eq(uuid)),
                    (prompts::instruction.eq(&body.system_content)),
                    (prompts::prompt_token.eq(openai_chat_res.usage.prompt_tokens as i32)),
                    (prompts::completion_token.eq(openai_chat_res.usage.completion_tokens as i32)),
                    (prompts::prompt_text.eq(&body.user_content)),
                    (prompts::completion_text.eq(&chat_choice.message.content)),
                    (prompts::total_token.eq(openai_chat_res.usage.total_tokens as i32)),
                    (prompts::total_cost.eq((openai_chat_res.usage.total_tokens / 2) as f64)),
                    (prompts::prompt_type.eq(prompt_type)),
                );
            })
            .collect();

        diesel::insert_into(prompts::table)
            .values(data)
            .get_results(&mut conn)
    }

    pub async fn new_instruct(
        pool: &web::Data<PgPool>,
        body: &web::Json<NewPromptRequestBody>,
    ) -> Result<Vec<Prompt>, String> {
        let openai_request_body = OpenAiChat::new(body);
        let openai = OpenAi::new(OpenAiEndpointType::ChatCompletion, openai_request_body);

        let openai_response = openai.request::<OpenAiChatResponse>().await;

        match openai_response {
            Ok(openai_chat_res) => {
                let insert_result = Self::new_instruct_insert(pool, body, openai_chat_res);

                match insert_result {
                    Ok(prompt_vec) => Ok(prompt_vec),
                    Err(diesel_error) => Err(diesel_error.to_string()),
                }
            }
            Err(reqwest_error) => Err(reqwest_error.to_string()),
        }
    }

    pub fn new_image_to_text_insert(
        pool: &web::Data<PgPool>,
        body: &web::Json<NewPromptRequestBody>,
    ) -> Result<Prompt, String> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();

        let data = (
            (prompts::user_id.eq(uuid)),
            (prompts::prompt_token.eq(0)),
            (prompts::completion_token.eq(0)),
            (prompts::prompt_text.eq("Image to text".to_string())),
            (prompts::completion_text.eq("".to_string())),
            (prompts::total_token.eq(0)),
            (prompts::total_cost.eq(0.0)),
            (prompts::instruction.eq("Image to text".to_string())),
            (prompts::prompt_type.eq(body.prompt_type.to_string())),
        );

        let insert_result = diesel::insert_into(prompts::table)
            .values(data)
            .get_result(&mut conn);

        match insert_result {
            Ok(prompt) => Ok(prompt),
            Err(diesel_error) => Err(diesel_error.to_string()),
        }
    }

    pub fn update_image_to_text_data(
        pool: &web::Data<PgPool>,
        body: &web::Json<UpdateImageToTextRequestBody>,
    ) -> Result<Prompt, String> {
        let mut conn = pool.get().unwrap();
        let user_id = uuid::Uuid::parse_str(&body.user_id).unwrap();

        let completion_token = body.completion_text.split(" ").collect::<Vec<&str>>().len();
        let updated_column = (
            prompts::completion_text.eq(&body.completion_text),
            prompts::completion_token.eq(completion_token as i32),
            prompts::total_token.eq(completion_token as i32),
            prompts::total_cost.eq((completion_token / 2) as f64),
        );

        let update_result = diesel::update(prompts::table)
            .filter(
                prompts::id
                    .eq(&body.prompt_id)
                    .and(prompts::user_id.eq(user_id)),
            )
            .set(updated_column)
            .get_result(&mut conn);

        match update_result {
            Ok(prompt) => Ok(prompt),
            Err(diesel_error) => Err(diesel_error.to_string()),
        }
    }

    pub fn new_text_to_speech_insert(
        pool: &web::Data<PgPool>,
        body: &web::Json<NewPromptRequestBody>,
    ) -> Result<Prompt, String> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
        let prompt_token = body.user_content.split(" ").collect::<Vec<&str>>().len();

        let data = (
            (prompts::user_id.eq(uuid)),
            (prompts::prompt_token.eq(prompt_token as i32)),
            (prompts::completion_token.eq(0)),
            (prompts::prompt_text.eq(&body.user_content)),
            (prompts::completion_text.eq("".to_string())),
            (prompts::total_token.eq(prompt_token as i32)),
            (prompts::total_cost.eq((prompt_token / 2) as f64)),
            (prompts::instruction.eq("Text to Speech".to_string())),
            (prompts::prompt_type.eq(body.prompt_type.to_string())),
        );

        let insert_result = diesel::insert_into(prompts::table)
            .values(data)
            .get_result(&mut conn);

        match insert_result {
            Ok(prompt) => Ok(prompt),
            Err(diesel_error) => Err(diesel_error.to_string()),
        }
    }

    pub async fn new_text_to_speech(
        pool: &web::Data<PgPool>,
        body: &web::Json<NewPromptRequestBody>,
    ) -> Result<Prompt, String> {
        let openai_request_body = OpenAiAudioSpeech::new(&body.user_content);
        let openai = OpenAi::new(OpenAiEndpointType::AudioSpeech, openai_request_body);
        let openai_result = openai.request_bytes().await;

        match openai_result {
            Ok(bytes) => {
                let insert_result = Self::new_text_to_speech_insert(pool, body);
                match insert_result {
                    Ok(prompt) => {
                        let file_name = format!("{}.mp3", &prompt.id);
                        let file_path = format!("/tmp/{}", file_name);
                        let file_creation = std::fs::write(file_path, &bytes);

                        match file_creation {
                            Ok(_) => Ok(prompt),
                            Err(msg) => Err(msg.to_string()),
                        }
                    }
                    Err(msg) => Err(msg),
                }
            }
            Err(msg) => Err(msg.to_string()),
        }
    }
}
