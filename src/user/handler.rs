use super::req::LoginReq;
use super::{model::User, req::UpdateUserReq};
use crate::db::PgPool;
use crate::subscription::model::Subscription;
use actix_web::{post, put, web, HttpResponse};

#[post("/login/gmail/")]
async fn gmail_login(pool: web::Data<PgPool>, body: web::Json<LoginReq>) -> HttpResponse {
    let login_user = User::check_user(pool.clone(), body.email.clone());

    match login_user {
        Ok(user) => {
            let subscription_result = Subscription::view_user_subscriptions(pool, user.id);
            let user_subscriptions = match subscription_result {
                Ok(subscriptions) => subscriptions,
                Err(_) => vec![],
            };
            User::send_token_response(user, user_subscriptions)
        }
        Err(_) => {
            let add_user = User::create(pool, body);
            match add_user {
                Ok(user) => User::send_token_response(user, vec![]),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
            }
        }
    }
}

#[put("/")]
async fn update_user(pool: web::Data<PgPool>, body: web::Json<UpdateUserReq>) -> HttpResponse {
    let update_user = User::update(pool.clone(), body);

    match update_user {
        Ok(user) => {
            let subscription_result = Subscription::view_user_subscriptions(pool, user.id);
            let user_subscriptions = match subscription_result {
                Ok(subscriptions) => subscriptions,
                Err(_) => vec![],
            };
            User::send_token_response(user, user_subscriptions)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(gmail_login).service(update_user);
}
