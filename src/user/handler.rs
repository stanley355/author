use super::req::LoginReq;
use super::{model::User, req::UpdateUserReq};
use crate::db::PgPool;
use actix_web::{post, put, web, HttpResponse};

#[post("/login/gmail/")]
async fn gmail_login(pool: web::Data<PgPool>, body: web::Json<LoginReq>) -> HttpResponse {
    let login_user = User::check_user(pool.clone(), body.email.clone());

    match login_user {
        Ok(user) => User::send_token_response(user),
        Err(_) => {
            let add_user = User::create(pool, body);
            match add_user {
                Ok(user) => User::send_token_response(user),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
            }
        }
    }
}

#[put("/")]
async fn update_user(pool: web::Data<PgPool>, body: web::Json<UpdateUserReq>) -> HttpResponse {
    let update_user = User::update(pool, body);

    match update_user {
        Ok(user) => {
            let insensitive_data = User::remove_sensitive_data(user);
            HttpResponse::Ok().json(insensitive_data)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(gmail_login).service(update_user);
}
