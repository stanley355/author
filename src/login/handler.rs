use super::model::LoginUser;
use super::req::LoginReq;
use crate::db::PgPool;
use actix_web::{post, web, HttpResponse};

#[post("/gmail/")]
async fn gmail_login(pool: web::Data<PgPool>, body: web::Json<LoginReq>) -> HttpResponse {
    let login_user = LoginUser::check_user(pool.clone(), body.email.clone());

    match login_user {
        Ok(user) => LoginUser::send_token_response(user),
        Err(_) => {
            let add_user = LoginUser::add(pool, body);
            match add_user {
                Ok(user) => LoginUser::send_token_response(user),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
            }
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(gmail_login);
}
