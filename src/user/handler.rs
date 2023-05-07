use super::model::User;
use super::req::*;
use super::res::{ErrorRes, LoginTokenRes};
use crate::db::PgPool;
use actix_web::{get, post, web, HttpResponse};
use bcrypt::verify;

#[get("")]
async fn get_user(pool: web::Data<PgPool>, query: web::Query<GetUserParam>) -> HttpResponse {
    // TODO: Create more param handler later
    let email = query.email.clone().unwrap();
    let user_exist = User::find_by_email(&pool, &email);

    match user_exist {
        Ok(user) => {
            let response = User::remove_password_field(user);
            HttpResponse::Ok().json(response)
        }
        Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
            error: err.to_string(),
            message: "Something went wrong".to_string(),
        }),
    }
}

#[post("/register/")]
async fn register(pool: web::Data<PgPool>, body: web::Json<RegisterReq>) -> HttpResponse {
    let user_exist = User::find_by_email(&pool, &body.email);

    match user_exist {
        Ok(_) => HttpResponse::BadRequest().json(ErrorRes {
            error: "User with the same email already exists".to_string(),
            message: "User with the same email already exists".to_string(),
        }),
        Err(_) => {
            let add_result = User::add_from_register(&pool, body);

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

#[post("/login/")]
async fn login(pool: web::Data<PgPool>, body: web::Json<LoginReq>) -> HttpResponse {
    let user_exist = User::find_by_email(&pool, &body.email);

    match user_exist {
        Ok(user) => {
            let password_valid = verify(&body.password, &user.password).unwrap();

            match password_valid {
                true => {
                    let token = User::create_login_token(user);
                    HttpResponse::Ok().json(LoginTokenRes { token })
                }
                false  => HttpResponse::BadRequest().json(ErrorRes {
                    error: "Password not matched".to_string(),
                    message: "Password not match".to_string()
                })
            }
        }
        Err(err) => HttpResponse::BadRequest().json(ErrorRes {
            error: err.to_string(),
            message: "User doesn't exist".to_string(),
        }),
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
    config
        .service(get_user)
        .service(register)
        .service(login)
        .service(gmail_login);
}
