use super::request::{PromptType, UpdateImageToTextRequestBody};
use crate::v2::http_error_response::HttpErrorResponse;
use crate::v2::prompt::model::Prompt;
use crate::v2::prompt::prompt_payment::PromptPayment;
use crate::v2::prompt::request::NewPromptRequestBody;
use crate::{db::PgPool, v2::user::model::User};
use actix_web::{post, put, web, HttpResponse};

#[post("/")]
async fn new_prompt(
    pool: web::Data<PgPool>,
    body: web::Json<NewPromptRequestBody>,
) -> HttpResponse {
    let prompt_payment = User::check_prompt_payment(&pool, &body.user_id, &body.prompt_type);

    return match prompt_payment {
        PromptPayment::PaymentRequired => HttpErrorResponse::payment_required(),
        PromptPayment::Balance => {
            if let PromptType::ImageToText = &body.prompt_type {
                let image_to_text_prompt_result = Prompt::new_image_to_text_insert(&pool, &body);
                return match image_to_text_prompt_result {
                    Ok(prompt) => HttpResponse::Ok().json(prompt),
                    Err(msg) => HttpErrorResponse::internal_server_error(msg),
                };
            }

            let prompt_result = match &body.prompt_type {
                PromptType::TextToSpeech => Prompt::new_text_to_speech(&pool, &body).await,
                _ => Prompt::new_instruct(&pool, &body).await,
            };

            match prompt_result {
                Ok(prompt) => {
                    // Reduce user balance credit by 0.5 per token
                    let _user = User::reduce_balance(&pool, &body.user_id, &prompt.total_cost);

                    HttpResponse::Ok().json(prompt)
                }
                Err(msg) => HttpErrorResponse::internal_server_error(msg),
            }
        }
        _ => {
            let prompt_result = match &body.prompt_type {
                PromptType::ImageToText => Prompt::new_image_to_text_insert(&pool, &body),
                PromptType::TextToSpeech => Prompt::new_text_to_speech(&pool, &body).await,
                _ => Prompt::new_instruct(&pool, &body).await,
            };

            match prompt_result {
                Ok(prompt) => HttpResponse::Ok().json(prompt),
                Err(msg) => HttpErrorResponse::internal_server_error(msg),
            }
        }
    };
}

#[put("/image-to-text/")]
async fn update_image_to_text_prompt(
    pool: web::Data<PgPool>,
    body: web::Json<UpdateImageToTextRequestBody>,
) -> HttpResponse {
    let update_result = Prompt::update_image_to_text_data(&pool, &body);

    match update_result {
        Ok(prompt) => {
            let prompt_payment =
                User::check_prompt_payment(&pool, &body.user_id, &PromptType::ImageToText);
            if let PromptPayment::Balance = prompt_payment {
                let _user = User::reduce_balance(&pool, &body.user_id, &prompt.total_cost);
            }

            HttpResponse::Ok().json(prompt)
        }
        Err(msg) => HttpErrorResponse::internal_server_error(msg),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(new_prompt)
        .service(update_image_to_text_prompt);
}
