use actix_web::{web, HttpResponse};

use super::model::Prompt;
use super::req::{UpdateImageToTextPromptReq, UpdateSpeechToTextPromptReq};
use crate::subscription::model::Subscription;
use crate::user::model::User;
use crate::{db::PgPool, util::web_response::WebErrorResponse};

pub enum PromptUpdateHandler {
    ImageToText(web::Json<UpdateImageToTextPromptReq>),
    SpeechToText(web::Json<UpdateSpeechToTextPromptReq>),
}

impl PromptUpdateHandler {
    pub async fn response_http(self, pool: web::Data<PgPool>, user_id: &str) -> HttpResponse {
        let subscription_result = Subscription::find_active_subscription(&pool, user_id);

        match subscription_result {
            Ok(_) => self.match_response(pool, false).await,
            Err(_) => {
                let user_result = User::find_by_id(&pool, user_id);

                match user_result {
                    Ok(user) => self.match_response(pool, user.balance > 0.0).await,
                    Err(err) => {
                        let err_res = WebErrorResponse::server_error(err, "User not found");
                        return HttpResponse::BadRequest().json(err_res);
                    }
                }
            }
        }
    }

    pub async fn match_response(
        self,
        pool: web::Data<PgPool>,
        is_pay_as_you_go: bool,
    ) -> HttpResponse {
        match self {
            PromptUpdateHandler::ImageToText(req_body) => {
                Prompt::update_image_to_text_response(&pool, req_body, is_pay_as_you_go).await
            }
            PromptUpdateHandler::SpeechToText(req_body) => {
                Prompt::update_speech_to_text_response(pool, req_body, is_pay_as_you_go).await
            }
        }
    }
}
