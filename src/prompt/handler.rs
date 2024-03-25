use super::model::Prompt;
use super::req::{
    NewImageToTextPromptReq, NewPromptReq, NewTextToSpeechPromptReq, PromptType,
    UpdateImageToTextPromptReq,
};
use crate::{
    db::PgPool, subscription::model::Subscription, user::model::User,
    util::web_response::WebErrorResponse,
};
use actix_web::put;
use actix_web::{http::StatusCode, post, web, HttpResponse};

pub enum PromptHandler {
    TranslateGrammarCheck(NewPromptReq),
    ImageToText(NewImageToTextPromptReq),
    TextToSpeech(NewTextToSpeechPromptReq),
}

impl PromptHandler {
    pub async fn new(self, pool: web::Data<PgPool>, user_id: &str) -> HttpResponse {
        let subscription_result = Subscription::find_active_subscription(&pool, user_id);

        match subscription_result {
            Ok(_) => match self {
                PromptHandler::TranslateGrammarCheck(body) => {
                    Prompt::new_prompt_response(&pool, web::Json(body), false).await
                }
                PromptHandler::ImageToText(body) => {
                    Prompt::new_image_to_text_response(&pool, web::Json(body)).await
                }
                PromptHandler::TextToSpeech(body) => {
                    Prompt::new_text_to_speech_response(&pool, web::Json(body), false).await
                }
            },
            Err(_) => {
                let user_result = User::find_by_id(&pool, user_id);

                match user_result {
                    Ok(user) => match user.balance > 0.0 {
                        true => {
                            match self {
                                PromptHandler::TranslateGrammarCheck(body) => {
                                    Prompt::new_prompt_response(&pool, web::Json(body), true).await
                                }
                                PromptHandler::ImageToText(body) => {
                                    Prompt::new_image_to_text_response(&pool, web::Json(body)).await
                                }
                                PromptHandler::TextToSpeech(body) => {
                                    Prompt::new_text_to_speech_response(
                                        &pool,
                                        web::Json(body),
                                        true,
                                    )
                                    .await
                                }
                            }

                            // Prompt::new_prompt_response(&pool, body, true).await
                        }
                        false => match self {
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
                        },
                    },
                    Err(err) => {
                        let err_res = WebErrorResponse::server_error(err, "User not found");
                        return HttpResponse::BadRequest().json(err_res);
                    }
                }
            }
        }
    }
}

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
                    false => {
                        Prompt::new_monthly_prompt(
                            &pool,
                            &body.user_id,
                            &body.prompt_type,
                            PromptHandler::TranslateGrammarCheck(body.clone()),
                        )
                        .await
                    }
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
                    false => {
                        Prompt::new_monthly_prompt(
                            &pool,
                            &body.user_id,
                            &body.prompt_type,
                            PromptHandler::ImageToText(body.clone()),
                        )
                        .await
                    }
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
                Ok(user) => {
                    Prompt::update_image_to_text_response(&pool, body, user.balance > 0.0).await
                }
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
    let subscription_result = Subscription::find_active_subscription(&pool, &body.user_id);

    match subscription_result {
        Ok(_) => Prompt::new_text_to_speech_response(&pool, body, false).await,
        Err(_) => {
            let user_result = User::find_by_id(&pool, &body.user_id);

            match user_result {
                Ok(user) => match user.balance > 0.0 {
                    true => Prompt::new_text_to_speech_response(&pool, body, true).await,
                    false => {
                        Prompt::new_monthly_prompt(
                            &pool,
                            &body.user_id,
                            &PromptType::TextToSpeech,
                            PromptHandler::TextToSpeech(body.clone()),
                        )
                        .await
                    }
                },
                Err(err) => {
                    let err_res = WebErrorResponse::server_error(err, "User not found");
                    return HttpResponse::BadRequest().json(err_res);
                }
            }
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(new_prompt)
        .service(new_image_to_text_prompt)
        .service(update_image_to_text_prompt)
        .service(new_text_to_speech);
}
