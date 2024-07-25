use actix_web::{post, web, HttpResponse};


use crate::{db::PgPool, http_error::HttpError};

#[post("/")]
async fn post_subscription(
    pool: web::Data<PgPool>,
    // json_request: web::Json<UsersLoginGmailRequest>,
) -> HttpResponse {

  HttpResponse::Ok().body("woi")
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(post_subscription);
}
