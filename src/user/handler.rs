use actix_web::{get, post, web, HttpResponse};

use super::model::User;
use super::req::*;
use super::res::{ErrorRes, LoginTokenRes};
use crate::db::PgPool;
use crate::util::web_response::WebErrorResponse;

#[get("")]
async fn get_user(pool: web::Data<PgPool>, query: web::Query<GetUserParam>) -> HttpResponse {
    let user_exist = User::find_by_email(&pool, &query.email);

    match user_exist {
        Ok(user) => {
            let response = User::remove_password_field(user);
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            let error_res = WebErrorResponse::bad_request(err, "User not found");
            HttpResponse::BadRequest().json(error_res)
        }
    }
}

#[post("/login/gmail/")]
async fn gmail_login(pool: web::Data<PgPool>, body: web::Json<GmailLoginReq>) -> HttpResponse {
    let user_exist = User::find_by_email(&pool, &body.email);

    match user_exist {
        Ok(user) => {
            let token = User::create_login_token(user);
            HttpResponse::Ok().json(LoginTokenRes { token })
        }
        Err(_) => {
            let add_result = User::add_from_gmail(&pool, body);

            match add_result {
                Ok(user) => {
                    let token = User::create_login_token(user);
                    HttpResponse::Ok().json(LoginTokenRes { token })
                }
                Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
                    error: err.to_string(),
                    message: "Something went wrong".to_string(),
                }),
            }
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(get_user).service(gmail_login);
}
