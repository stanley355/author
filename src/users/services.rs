use actix_web::{get, post, put, web, HttpResponse};
use regex::Regex;
use std::env;

use super::jwt::UserJwt;
use super::model::User;
use super::request::{
    UsersAccountRequest, UsersChangePasswordRequest, UsersLoginGmailRequest, UsersLoginRequest,
    UsersRegisterRequest, UsersResetPasswordRequest,
};
use super::response::{UsersAccountResponse, UsersBaseResponse};
use crate::{db::PgPool, http_error::HttpError};

#[post("/login/")]
async fn post_login(
    pool: web::Data<PgPool>,
    request_json: web::Json<UsersLoginRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();

    return match User::find_by_email(&pool, &request.email) {
        Ok(user) => {
            let password_valid = user.check_password_valid(&request.password);
            match password_valid {
                true => {
                    let user_jwt = UserJwt::new(&user);
                    HttpResponse::Ok().json(user_jwt)
                }
                false => HttpError::bad_request("Invalid email or password"),
            }
        }
        Err(_) => HttpError::bad_request("User not found"),
    };
}

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
            let insert_result = User::new_login_gmail_insert(&pool, &request);
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

#[post("/register/")]
async fn post_register(
    pool: web::Data<PgPool>,
    request_json: web::Json<UsersRegisterRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();

    if &request.fullname.len() < &(4 as usize) {
        return HttpError::bad_request("Invalid fullname: 4 characters mininum");
    }

    let fullname_has_symbol = Regex::new(r"[^A-Za-z0-9\s]")
        .unwrap()
        .is_match(&request.fullname);
    if fullname_has_symbol {
        return HttpError::bad_request("Invalid fullname: Fullname can't contain symbol");
    }

    let email_is_invalid = !Regex::new(r"^[\w\.-]+@[a-zA-Z\d\.-]+\.[a-zA-Z]{2,}$")
        .unwrap()
        .is_match(&request.email);
    if email_is_invalid {
        return HttpError::bad_request("Invalid email: Format");
    }

    if &request.password.len() < &(4 as usize) {
        return HttpError::bad_request("Invalid password: 4 characters mininum");
    }

    if &request.password != &request.password_again {
        return HttpError::bad_request(
            "Invalid password: Password is not similar to password confirmation",
        );
    }

    if let Ok(_) = User::find_by_email(&pool, &request.email) {
        return HttpError::bad_request("User already exists");
    }

    return match User::new_register_insert(&pool, &request) {
        Ok(user) => {
            let user_jwt = UserJwt::new(&user);
            HttpResponse::Created().json(user_jwt)
        }
        Err(insert_error) => HttpError::internal_server_error(&insert_error.to_string()),
    };
}

pub fn services(config: &mut web::ServiceConfig) {
    config
        .service(post_login)
        .service(post_login_gmail)
        .service(get_account)
        .service(put_reset_password)
        .service(put_change_password)
        .service(post_register);
}
