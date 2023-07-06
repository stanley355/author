use super::model::Prompt;
use super::req::NewPromptReq;
use crate::{
    db::PgPool,
    user::{model::User, req::ReduceBalanceReq, res::ErrorRes},
};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new_prompt(pool: web::Data<PgPool>, body: web::Json<NewPromptReq>) -> HttpResponse {
    let result = Prompt::new(&pool, body);

    match result {
        Ok(checkbot) => HttpResponse::Accepted().json(checkbot),
        Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
            error: err.to_string(),
            message: "Internal Server error".to_string(),
        }),
    }
}

#[post("/premium/")]
async fn new_premium_prompt(
    pool: web::Data<PgPool>,
    body: web::Json<NewPromptReq>,
) -> HttpResponse {
    let prompt_result = Prompt::new_premium(&pool, &body);
    let reduce_payload = ReduceBalanceReq {
        user_id: body.user_id.clone(),
        reduce_amount: (body.prompt_token + body.completion_token) as f64,
    };
    let user_result = User::reduce_balance(&pool, &reduce_payload);

    match (prompt_result, user_result) {
        (Ok(prompt), Ok(_)) => HttpResponse::Accepted().json(prompt),
        _ => HttpResponse::InternalServerError().json(ErrorRes {
            error: "Something went wrong".to_string(),
            message: "Internal Server error".to_string(),
        }),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_prompt).service(new_premium_prompt);
}
