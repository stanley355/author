use actix_web::{post, web, HttpResponse};

use crate::db::PgPool;
use super::request::UsersLoginGmailRequest;

#[post("/login/gmail/")]
async fn post_login_gmail(
    pool: web::Data<PgPool>,
    body: web::Json<UsersLoginGmailRequest>,
) -> HttpResponse {
    HttpResponse::Ok().body("hi")
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(post_login_gmail);
}
