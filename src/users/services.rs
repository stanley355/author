use actix_web::{post, web, HttpResponse};

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
            let current_user = User::find_by_email(&pool, &request.email);
            HttpResponse::Ok().body("hi")
        }
    }
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(post_login_gmail);
}
