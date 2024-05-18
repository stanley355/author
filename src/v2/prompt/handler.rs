use super::request::{DeleteTtsFileQuery, PromptType, UpdateImageToTextRequestBody};
use super::response::PromptHttpResponse;
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
        PromptPayment::Balance => PromptHttpResponse::new(&pool, &body, true).await,
        _ => PromptHttpResponse::new(&pool, &body, false).await
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
