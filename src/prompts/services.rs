use actix_web::{get, post, web, HttpResponse};

use crate::{db::PgPool, http_error::HttpError};

#[post("/")]
async fn post_prompt(
    pool: web::Data<PgPool>,
    // request_json: web::Json<UsersLoginGmailRequest>,
) -> HttpResponse {
    HttpResponse::Ok().body("woi")
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(post_prompt);
}
