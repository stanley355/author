use super::model::Prompt;
use super::payment::PromptPayment;
use super::request::NewPromptRequest;
use super::request::PromptType;
use crate::{db::PgPool, http_error::HttpError};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn post_prompt(
    pool: web::Data<PgPool>,
    request_json: web::Json<NewPromptRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();

    return match &request.prompt_type {
        PromptType::Translate | PromptType::Checkbot | PromptType::PhoneticTranscriptions => {
            let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();
            let prompt_payment = Prompt::check_payment(&pool, &user_id, &request.prompt_type);

            match prompt_payment {
                PromptPayment::PaymentRequired => HttpError::payment_required(),
                _ => HttpResponse::Ok().body("woi"),
            }
        }
        _ => HttpError::bad_request(
            "Only Translate, Checkbot, and PhoneticTranscriptions prompt_type accepted",
        ),
    };
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(post_prompt);
}
