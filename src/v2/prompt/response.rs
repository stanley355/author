use super::request::{NewTextToSpeechRequestBody, NewTranscriptionsRequestBody};
use crate::v2::http_error_response::HttpErrorResponse;
use crate::v2::prompt::model::Prompt;
use crate::v2::prompt::request::NewPromptRequestBody;
use crate::{db::PgPool, v2::user::model::User};
use actix_web::{web, HttpResponse};

pub struct PromptHttpResponse;

impl PromptHttpResponse {
    pub async fn new_text_to_speech(
        pool: &web::Data<PgPool>,
        body: &web::Json<NewTextToSpeechRequestBody>,
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
                    let _user =
                        User::reduce_balance(&pool, &body.user_id, &prompt_vec[0].total_cost);
                }
                HttpResponse::Ok().json(prompt_vec)
            }
            Err(msg) => HttpErrorResponse::internal_server_error(msg),
        }
    }

    pub async fn new_transcriptions(
        pool: &web::Data<PgPool>,
        body: &web::Json<NewTranscriptionsRequestBody>,
    ) -> HttpResponse {
        let transcription_result = Prompt::new_transcriptions(&pool, &body).await;

        match transcription_result {
            Ok(transcription) => HttpResponse::Ok().json(transcription),
            Err(msg) => HttpErrorResponse::internal_server_error(msg),
        }
    }
}
