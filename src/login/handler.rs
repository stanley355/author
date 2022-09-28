use super::model::LoginUser;
use super::req::LoginReq;
use super::res::LoginRes;
use crate::db::PgPool;
use actix_web::{get, post, put, web, HttpResponse};

#[post("/")]
async fn login(pool: web::Data<PgPool>, body: web::Json<LoginReq>) -> HttpResponse {
    let login_user = LoginUser::check_user(pool, body.email.clone());

    match login_user {
        Ok(res) => {
            let token = LoginUser::hash_user_data(res);
            let res = LoginRes::new(token);
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            HttpResponse::BadRequest().body(format!("Missing Parameter: code, year {}", err))
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(login);
}
