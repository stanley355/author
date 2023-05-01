use super::req::GmailLoginReq;
use  super::res::ErrorRes;
use crate::db::PgPool;
use actix_web::{post, web, HttpResponse};

#[post("/login/gmail/")]
async fn gmail_login(pool: web::Data<PgPool>, body: web::Json<GmailLoginReq>) -> HttpResponse {
    println!("{:?} test: ", body);
    HttpResponse::Accepted().json(ErrorRes {
        error: "Error".to_string(),
        message: "salah woi".to_string()
    })
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(gmail_login);
}
