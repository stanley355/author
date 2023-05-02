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
            error: "Salah".to_string(),
            message: "Salah".to_string(),
        }),
        Err(_) => {
            let user = User::add(&pool, body);
            println!("{:?}: ", user.unwrap());

            HttpResponse::Ok().body(format!("hahahihi"))
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(gmail_login);
}
