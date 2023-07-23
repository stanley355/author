use super::req::{NewPromptReq, UpdatePromptReq};
use super::{model::Prompt, req::FindPromptReq};
use crate::{
    db::PgPool,
    user::{model::User, req::ReduceBalanceReq, res::ErrorRes},
};
use actix_web::{get, post, put, web, HttpResponse};

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

#[get("")]
async fn find_document_prompts(
    pool: web::Data<PgPool>,
    query: web::Query<FindPromptReq>,
) -> HttpResponse {
    let doc_id = query.doc_id.clone();

    match doc_id {
        Some(id) => {
            let document = Prompt::find_by_doc_id(&pool, &id);
            match document {
                Ok(doc) => HttpResponse::Ok().json(doc),
                Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
                    error: err.to_string(),
                    message: "Document not found, please try again".to_string(),
                }),
            }
        }
        None => HttpResponse::BadRequest().json(ErrorRes {
            error: "Missing or Wrong Parameter".to_string(),
            message: "Missing or Wrong Parameter".to_string(),
        }),
    }
}

#[put("/")]
async fn update_prompt(pool: web::Data<PgPool>, body: web::Json<UpdatePromptReq>) -> HttpResponse {
    let prompt = Prompt::update_prompt(&pool, &body);
    match prompt {
        Ok(prom) => HttpResponse::Ok().json(prom),
        Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
            error: err.to_string(),
            message: "Fail to update, please try again".to_string(),
        }),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(new_prompt)
        .service(new_premium_prompt)
        .service(find_document_prompts)
        .service(update_prompt);
}
