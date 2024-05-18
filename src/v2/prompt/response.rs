use super::request::PromptType;
use crate::v2::http_error_response::HttpErrorResponse;
use crate::v2::prompt::model::Prompt;
use crate::v2::prompt::request::NewPromptRequestBody;
use crate::{db::PgPool, v2::user::model::User};
use actix_web::{web, HttpResponse};

pub struct PromptHttpResponse;

impl PromptHttpResponse {
    pub async fn new(
        pool: &web::Data<PgPool>,
        body: &web::Json<NewPromptRequestBody>,
        is_balance_payment: bool,
    ) -> HttpResponse {
        return match &body.prompt_type {
            PromptType::ImageToText => PromptHttpResponse::new_image_to_text(&pool, &body),
            PromptType::TextToSpeech => {
                PromptHttpResponse::new_text_to_speech(&pool, &body, is_balance_payment).await
            }
            _ => PromptHttpResponse::new_instruct(&pool, &body, is_balance_payment).await,
        };
    }

    pub fn new_image_to_text(
        pool: &web::Data<PgPool>,
        body: &web::Json<NewPromptRequestBody>,
    ) -> HttpResponse {
        let image_to_text_prompt_result = Prompt::new_image_to_text_insert(&pool, &body);
        return match image_to_text_prompt_result {
            Ok(prompt) => HttpResponse::Ok().json(prompt),
            Err(msg) => HttpErrorResponse::internal_server_error(msg),
        };
    }

    pub async fn new_text_to_speech(
        pool: &web::Data<PgPool>,
        body: &web::Json<NewPromptRequestBody>,
        is_balance_payment: bool,
    ) -> HttpResponse {
        let prompt_tts_result = Prompt::new_text_to_speech(&pool, &body).await;
        match prompt_tts_result {
            Ok(prompt) => {
                if is_balance_payment {
                    let _user = User::reduce_balance(&pool, &body.user_id, &prompt.total_cost);
                }
                HttpResponse::Ok().json(prompt)
            }
            Err(msg) => HttpErrorResponse::internal_server_error(msg),
        }
    }

    pub async fn new_instruct(
        pool: &web::Data<PgPool>,
        body: &web::Json<NewPromptRequestBody>,
        is_balance_payment: bool,
    ) -> HttpResponse {
        let prompt_result = Prompt::new_instruct(&pool, &body).await;
        match prompt_result {
            Ok(prompt_vec) => {
                if is_balance_payment {
                    let total_cost = prompt_vec
                        .iter()
                        .map(|prompt| prompt.total_cost)
                        .reduce(|a, b| a + b);
                    let _user = User::reduce_balance(&pool, &body.user_id, &total_cost.unwrap());
                }
                HttpResponse::Ok().json(prompt_vec)
            }
            Err(msg) => HttpErrorResponse::internal_server_error(msg),
        }
    }
}
