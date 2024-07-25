use actix_web::{post, web, HttpResponse};

use super::jwt::UserJwt;
use super::model::User;
use super::request::UsersLoginGmailRequest;
use crate::{db::PgPool, http_error::HttpError};

#[post("/login/gmail/")]
async fn post_login_gmail(
    pool: web::Data<PgPool>,
    json_request: web::Json<UsersLoginGmailRequest>,
) -> HttpResponse {
    let request = json_request.into_inner();

    match request.is_valid() {
        false => HttpError::bad_request("Bad Request"),
        true => {
            if let Ok(user) = User::find_by_email(&pool, &request.email) {
                let user_jwt = UserJwt::new(&user);
                HttpResponse::Ok().json(user_jwt)
            } else {
                let insert_result = User::insert_new_from_login_gmail(&pool, &request);
                match insert_result {
                    Ok(new_user) => {
                        let new_user_jwt = UserJwt::new(&new_user);
                        HttpResponse::Ok().json(new_user_jwt)
                    }
                    Err(diesel_error) => {
                        HttpError::internal_server_error(&diesel_error.to_string())
                    }
                }
            }
        }
    }
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(post_login_gmail);
}
