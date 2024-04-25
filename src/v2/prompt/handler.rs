use crate::v2::http_error_response::HttpErrorResponse;
use crate::v2::prompt::model::Prompt;
use crate::v2::prompt::prompt_payment::PromptPayment;
use crate::v2::prompt::request::NewPromptRequestBody;
use crate::{db::PgPool, v2::user::model::User};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new_prompt(
    pool: web::Data<PgPool>,
    body: web::Json<NewPromptRequestBody>,
) -> HttpResponse {
    let prompt_payment = User::check_prompt_payment(&pool, &body.user_id, &body.prompt_type);

    return match prompt_payment {
        PromptPayment::PaymentRequired=> HttpErrorResponse::payment_required(),
        _ => {
            let prompt_result = Prompt::new_instruct(&pool, &body).await;

            match prompt_result {
                Ok(prompt) => HttpResponse::Ok().json(prompt),
                Err(msg) => HttpErrorResponse::internal_server_error(msg),
            }
        }
    };
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_prompt);
}
