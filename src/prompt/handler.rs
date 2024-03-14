use super::model::Prompt;
use super::req::NewPromptReq;
use crate::{db::PgPool, util::web_response::WebErrorResponse};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new_prompt(pool: web::Data<PgPool>, body: web::Json<NewPromptReq>) -> HttpResponse {
    let result = Prompt::new(&pool, body).await;

    match result {
        Ok(new_prompt_res) => HttpResponse::Ok().json(new_prompt_res),
        Err(err) => {
            let err_res = WebErrorResponse::reqwest_server_error(err, "Fail to execute, please try again");
            return  HttpResponse::InternalServerError().json(err_res);
        },
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_prompt);
}
