use super::req::GmailLoginReq;
use crate::db::PgPool;
use actix_web::{post, web, HttpResponse};

#[post("/login/gmail/")]
async fn gmail_login(pool: web::Data<PgPool>, body: web::Json<GmailLoginReq>) -> HttpResponse {
    println!("{:?} test: ", body);
    HttpResponse::Accepted().body(format!("hihi"))
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(gmail_login);
}
