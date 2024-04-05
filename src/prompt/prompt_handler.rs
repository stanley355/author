use super::model::Prompt;
use super::req::{NewImageToTextPromptReq, NewPromptReq, NewTextToSpeechPromptReq, PromptType};
use crate::student::model::Student;
use crate::util::http_error_response::HttpErrorResponse;
use crate::{db::PgPool, subscription::model::Subscription, user::model::User};
use actix_web::{web, HttpResponse};

pub enum PromptHandler {
    TranslateGrammarCheck(NewPromptReq),
    ImageToText(NewImageToTextPromptReq),
    TextToSpeech(NewTextToSpeechPromptReq),
}

impl PromptHandler {
    pub async fn new(self, pool: web::Data<PgPool>, user_id: &str) -> HttpResponse {
        let student_result = Student::find_active_discount(&pool, user_id);

        match student_result {
            Ok(student) => {
                let student_disc = student.check_discount_availability();

                match (student_disc.is_student, student_disc.is_free_discount) {
                    (true, true) => Self::has_subscription_response(self, pool, false).await,
                    (_, _) => Self::is_not_student_response(self, pool, user_id).await,
                }
            }
            Err(_) => Self::is_not_student_response(self, pool, user_id).await,
        }
    }

    pub async fn is_not_student_response(
        self,
        pool: web::Data<PgPool>,
        user_id: &str,
    ) -> HttpResponse {
        let subscription_result = Subscription::find_active_subscription(&pool, user_id);

        match subscription_result {
            Ok(_) => Self::has_subscription_response(self, pool, false).await,
            Err(_) => {
                let user_result = User::find_by_id(&pool, user_id);

                match user_result {
                    Ok(user) => match user.balance > 0.0 {
                        true => Self::has_subscription_response(self, pool, true).await,
                        false => Self::monthly_prompt_response(self, pool).await,
                    },
                    Err(err) => {
                        let msg = "User not found";
                        HttpErrorResponse::new(None, err.to_string(), msg).response()
                    }
                }
            }
        }
    }

    pub async fn has_subscription_response(
        self,
        pool: web::Data<PgPool>,
        is_pay_as_you_go: bool,
    ) -> HttpResponse {
        match self {
            PromptHandler::TranslateGrammarCheck(body) => {
                Prompt::new_prompt_response(&pool, web::Json(body), is_pay_as_you_go).await
            }
            PromptHandler::ImageToText(body) => {
                Prompt::new_image_to_text_response(&pool, web::Json(body)).await
            }
            PromptHandler::TextToSpeech(body) => {
                Prompt::new_text_to_speech_response(&pool, web::Json(body), is_pay_as_you_go).await
            }
        }
    }

    pub async fn monthly_prompt_response(self, pool: web::Data<PgPool>) -> HttpResponse {
        match self {
            PromptHandler::TranslateGrammarCheck(body) => {
                Prompt::new_monthly_prompt(
                    &pool,
                    &body.user_id,
                    &body.prompt_type,
                    PromptHandler::TranslateGrammarCheck(body.clone()),
                )
                .await
            }
            PromptHandler::ImageToText(body) => {
                Prompt::new_monthly_prompt(
                    &pool,
                    &body.user_id,
                    &PromptType::ImageToText,
                    PromptHandler::ImageToText(body.clone()),
                )
                .await
            }
            PromptHandler::TextToSpeech(body) => {
                Prompt::new_monthly_prompt(
                    &pool,
                    &body.user_id,
                    &PromptType::TextToSpeech,
                    PromptHandler::TextToSpeech(body.clone()),
                )
                .await
            }
        }
    }
}
