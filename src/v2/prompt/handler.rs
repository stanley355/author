use super::request::{DeleteTtsFileQuery, PromptType, UpdateImageToTextRequestBody};
use crate::v2::http_error_response::HttpErrorResponse;
use crate::v2::prompt::model::Prompt;
use crate::v2::prompt::prompt_payment::PromptPayment;
use crate::v2::prompt::request::NewPromptRequestBody;
use crate::{db::PgPool, v2::user::model::User};
use actix_web::{delete, post, put, web, HttpResponse};

#[post("/")]
async fn new_prompt(
    pool: web::Data<PgPool>,
    body: web::Json<NewPromptRequestBody>,
) -> HttpResponse {
    let prompt_payment = User::check_prompt_payment(&pool, &body.user_id, &body.prompt_type);

    return match prompt_payment {
        PromptPayment::PaymentRequired => HttpErrorResponse::payment_required(),
        PromptPayment::Balance => {
            return match &body.prompt_type {
                PromptType::ImageToText => {
                    let image_to_text_prompt_result =
                        Prompt::new_image_to_text_insert(&pool, &body);
                    return match image_to_text_prompt_result {
                        Ok(prompt) => HttpResponse::Ok().json(prompt),
                        Err(msg) => HttpErrorResponse::internal_server_error(msg),
                    };
                }
                PromptType::TextToSpeech => {
                    let prompt_tts_result = Prompt::new_text_to_speech(&pool, &body).await;
                    match prompt_tts_result {
                        Ok(prompt) => {
                            let _user =
                                User::reduce_balance(&pool, &body.user_id, &prompt.total_cost);
                            HttpResponse::Ok().json(prompt)
                        }
                        Err(msg) => HttpErrorResponse::internal_server_error(msg),
                    }
                }
                _ => {
                    // GrammarCheck or Translate
                    let prompt_result = Prompt::new_instruct(&pool, &body).await;
                    match prompt_result {
                        Ok(prompt_vec) => {
                            let total_cost = prompt_vec
                                .iter()
                                .map(|prompt| prompt.total_cost)
                                .reduce(|a, b| a + b);
                            let _user =
                                User::reduce_balance(&pool, &body.user_id, &total_cost.unwrap());

                            HttpResponse::Ok().json(prompt_vec)
                        }
                        Err(msg) => HttpErrorResponse::internal_server_error(msg),
                    }
                }
            };
        }
        _ => {
            return match &body.prompt_type {
                PromptType::GrammarCheck | PromptType::Translate => {
                    let prompt_vec_result = Prompt::new_instruct(&pool, &body).await;
                    match prompt_vec_result {
                        Ok(prompt) => HttpResponse::Ok().json(prompt),
                        Err(msg) => HttpErrorResponse::internal_server_error(msg),
                    }
                }
                PromptType::ImageToText => {
                    let prompt_result = Prompt::new_image_to_text_insert(&pool, &body);
                    match prompt_result {
                        Ok(prompt) => HttpResponse::Ok().json(prompt),
                        Err(msg) => HttpErrorResponse::internal_server_error(msg),
                    }
                }
                PromptType::TextToSpeech => {
                    let prompt_result = Prompt::new_text_to_speech(&pool, &body).await;
                    match prompt_result {
                        Ok(prompt) => HttpResponse::Ok().json(prompt),
                        Err(msg) => HttpErrorResponse::internal_server_error(msg),
                    }
                }
            };
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

#[delete("/tts/file")]
async fn delete_tts_file(query: web::Query<DeleteTtsFileQuery>) -> HttpResponse {
    let file_path = format!("/tmp/{}.mp3", &query.prompt_id);
    let file_del_result = std::fs::remove_file(file_path);

    match file_del_result {
        Ok(_) => HttpResponse::Ok().body(query.prompt_id.to_string()),
        Err(err) => HttpErrorResponse::internal_server_error(err.to_string()),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(new_prompt)
        .service(update_image_to_text_prompt)
        .service(delete_tts_file);
}
