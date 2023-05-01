use super::req::LoginReq;
use crate::db::PgPool;
use actix_web::{post, web, HttpResponse};

#[post("/login/gmail/")]
async fn gmail_login(pool: web::Data<PgPool>, body: web::Json<LoginReq>) -> HttpResponse {}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(gmail_login);
}
