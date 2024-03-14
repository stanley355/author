use super::model::Prompt;
use super::req::NewPromptReq;
use crate::{db::PgPool, user::res::ErrorRes};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new_prompt(pool: web::Data<PgPool>, body: web::Json<NewPromptReq>) -> HttpResponse {
    let result = Prompt::new(&pool, body).await;

        HttpResponse::Accepted().body("woi".to_string())
    // match result {
    //     Ok(checkbot) => 
    //     HttpResponse::Accepted().body("woi".to_string())
    //     ,
    //     Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
    //         error: err.to_string(),
    //         message: "Internal Server error".to_string(),
    //     }),
    // }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_prompt);
}
