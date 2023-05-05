use super::model::User;
use super::req::GmailLoginReq;
use super::res::ErrorRes;
use crate::db::PgPool;
use actix_web::{post, web, HttpResponse};

#[post("/login/gmail/")]
async fn gmail_login(pool: web::Data<PgPool>, body: web::Json<GmailLoginReq>) -> HttpResponse {
    let email_exist = User::find_by_email(&pool, &body.email);

    match email_exist {
        Ok(_) => HttpResponse::Accepted().json(ErrorRes {
            error: "Email Terdaftar".to_string(),
            message: "Terdaftar".to_string(),
        }),
        Err(_) => {
            let result = User::add(&pool, body);

            match result {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
                    error: err.to_string(),
                    message: "Something went wrong".to_string(),
                }),
            }
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(gmail_login);
}
