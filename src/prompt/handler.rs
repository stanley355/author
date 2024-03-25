use std::io::{Cursor, Write};

use super::model::Prompt;
use super::req::{
    NewImageToTextPromptReq, NewPromptReq, NewTextToSpeechPromptReq, PromptType,
    UpdateImageToTextPromptReq,
};
use crate::openai::model::{OpenAi, OpenAiTextToSpeechReq};
use crate::{
    db::PgPool, subscription::model::Subscription, user::model::User,
    util::web_response::WebErrorResponse,
};
use actix_web::put;
use actix_web::{http::StatusCode, post, web, HttpResponse};

#[post("/")]
async fn new_prompt(pool: web::Data<PgPool>, body: web::Json<NewPromptReq>) -> HttpResponse {
    let subscription_result = Subscription::find_active_subscription(&pool, &body.user_id);

    match subscription_result {
        Ok(_) => Prompt::new_prompt_response(&pool, body, false).await,
        Err(_) => {
            let user_result = User::find_by_id(&pool, &body.user_id);

            match user_result {
                Ok(user) => match user.balance > 0.0 {
                    true => Prompt::new_prompt_response(&pool, body, true).await,
                    false => Prompt::new_monthly_prompt(&pool, body).await,
                },
                Err(err) => {
                    let err_res = WebErrorResponse::server_error(err, "User not found");
                    return HttpResponse::BadRequest().json(err_res);
                }
            }
        }
    }
}

#[post("/image-to-text/")]
async fn new_image_to_text_prompt(
    pool: web::Data<PgPool>,
    body: web::Json<NewImageToTextPromptReq>,
) -> HttpResponse {
    if body.prompt_type.to_string() != PromptType::ImageToText.to_string() {
        let err_res = WebErrorResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            error: "Invalid Prompt Type".to_string(),
            message: "Invalid Prompt Type".to_string(),
        };

        return HttpResponse::BadRequest().json(err_res);
    }

    let subscription_result = Subscription::find_active_subscription(&pool, &body.user_id);

    match subscription_result {
        Ok(_) => Prompt::new_image_to_text_response(&pool, body).await,
        Err(_) => {
            let user_result = User::find_by_id(&pool, &body.user_id);

            match user_result {
                Ok(user) => match user.balance > 0.0 {
                    true => Prompt::new_image_to_text_response(&pool, body).await,
                    false => Prompt::new_image_to_text_monthly_prompt(&pool, body).await,
                },
                Err(err) => {
                    let err_res = WebErrorResponse::server_error(err, "User not found");
                    return HttpResponse::BadRequest().json(err_res);
                }
            }
        }
    }
}

#[put("/image-to-text/")]
async fn update_image_to_text_prompt(
    pool: web::Data<PgPool>,
    body: web::Json<UpdateImageToTextPromptReq>,
) -> HttpResponse {
    let subscription_result = Subscription::find_active_subscription(&pool, &body.user_id);

    match subscription_result {
        Ok(_) => Prompt::update_image_to_text_response(&pool, body, false).await,
        Err(_) => {
            let user_result = User::find_by_id(&pool, &body.user_id);

            match user_result {
                Ok(_) => Prompt::update_image_to_text_response(&pool, body, true).await,
                Err(err) => {
                    let err_res = WebErrorResponse::server_error(err, "User not found");
                    return HttpResponse::BadRequest().json(err_res);
                }
            }
        }
    }
}

#[post("/text-to-speech/")]
async fn new_text_to_speech(
    pool: web::Data<PgPool>,
    body: web::Json<NewTextToSpeechPromptReq>,
) -> HttpResponse {
    let file_req_body = OpenAiTextToSpeechReq::new(&body);
    let file_byte_res = OpenAi::new_text_to_speech(file_req_body).await;

    match file_byte_res {
        Ok(bytes) => HttpResponse::Ok().body(bytes),
        Err(_) => HttpResponse::BadRequest().body("woi".to_string()),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(new_prompt)
        .service(new_image_to_text_prompt)
        .service(update_image_to_text_prompt)
        .service(new_text_to_speech);
}
