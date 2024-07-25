use actix_web::{post, web, HttpResponse};

use crate::{db::PgPool, http_error::HttpError};
use super::request::UsersLoginGmailRequest;

#[post("/login/gmail/")]
async fn post_login_gmail(
    pool: web::Data<PgPool>,
    body: web::Json<UsersLoginGmailRequest>,
) -> HttpResponse {
    let is_valid = &body.into_inner().is_valid();

    match is_valid {
       false => HttpError::bad_request("Bad Request"),
       true  => HttpResponse::Ok().body("hi")
    }
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(post_login_gmail);
}
