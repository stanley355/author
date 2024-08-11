use actix_web::{get, post, put, web, HttpResponse};
use std::env;

use super::jwt::UserJwt;
use super::model::User;
use super::request::{
    UsersAccountRequest, UsersChangePasswordRequest, UsersLoginGmailRequest,
    UsersResetPasswordRequest,
};
use super::response::{UsersAccountResponse, UsersBaseResponse};
use crate::{db::PgPool, http_error::HttpError};

#[post("/login/gmail/")]
async fn post_login_gmail(
    pool: web::Data<PgPool>,
    request_json: web::Json<UsersLoginGmailRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();

    match User::find_by_email(&pool, &request.email) {
        Ok(user) => {
            let user_jwt = UserJwt::new(&user);
            HttpResponse::Ok().json(user_jwt)
        }
        Err(_) => {
            let insert_result = User::new_from_login_gmail_insert(&pool, &request);
            match insert_result {
                Ok(new_user) => {
                    let new_user_jwt = UserJwt::new(&new_user);
                    HttpResponse::Created().json(new_user_jwt)
                }
                Err(diesel_error) => HttpError::internal_server_error(&diesel_error.to_string()),
            }
        }
    }
}

#[get("/account")]
async fn get_account(
    pool: web::Data<PgPool>,
    request_query: web::Query<UsersAccountRequest>,
) -> HttpResponse {
    let user_id = uuid::Uuid::parse_str(&request_query.id).unwrap();

    let user_result = User::find(&pool, user_id);

    match user_result {
        Ok(user) => {
            let user_account = UsersAccountResponse::new(&pool, &user);
            HttpResponse::Ok().json(user_account)
        }
        Err(diesel_error) => HttpError::bad_request(&diesel_error.to_string()),
    }
}

#[put("/reset-password/")]
async fn put_reset_password(
    pool: web::Data<PgPool>,
    request_json: web::Json<UsersResetPasswordRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();
    let admin_password = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD not set");

    if request.admin_password != admin_password {
        return HttpError::bad_request("Invalid admin password");
    }

    let user_id = uuid::Uuid::parse_str(&request.id).unwrap();
    let user_result = User::change_password(&pool, &user_id, &request.new_password);
    match user_result {
        Ok(user) => {
            let base_user = UsersBaseResponse::new(&user);
            return HttpResponse::Ok().json(base_user);
        }
        Err(err) => HttpError::bad_request(&err.to_string()),
    }
}

#[put("/change-password/")]
async fn put_change_password(
    pool: web::Data<PgPool>,
    request_json: web::Json<UsersChangePasswordRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();
    let whitespaces: &[char] = &[' ', '\n', '\t'];

    if request.new_password.contains(whitespaces) {
        return HttpError::bad_request("New password can't contain whitespaces");
    }

    if request.new_password != request.new_password_again {
        return HttpError::bad_request("New password is not similar to new password confirmation");
    }

    let user_id = uuid::Uuid::parse_str(&request.id).unwrap();
    return match User::find(&pool, user_id) {
        Ok(user) => {
            let old_password_valid = user.check_password_valid(&request.old_password);
            match old_password_valid {
                true => {
                    let updated_user_result =
                        User::change_password(&pool, &user_id, &request.new_password);
                    match updated_user_result {
                        Ok(updated_user) => {
                            let base_user = UsersBaseResponse::new(&updated_user);
                            HttpResponse::Ok().json(base_user)
                        }
                        Err(change_password_error) => {
                            HttpError::internal_server_error(&change_password_error.to_string())
                        }
                    }
                }
                false => HttpError::bad_request("Invalid old password"),
            }
        }
        Err(find_user_error) => HttpError::bad_request(&find_user_error.to_string()),
    };
}

pub fn services(config: &mut web::ServiceConfig) {
    config
        .service(post_login_gmail)
        .service(get_account)
        .service(put_reset_password)
        .service(put_change_password);
}
