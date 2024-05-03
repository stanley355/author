use super::model::User;
use super::response::LoginResponse;
use super::user_insensitive::UserInsensitive;
use crate::v2::user::request::LoginGmailRequestBody;
use crate::{db::PgPool, v2::http_error_response::HttpErrorResponse};
use actix_web::{post, web, HttpResponse};

#[post("/login/gmail")]
async fn login_gmail(
    pool: web::Data<PgPool>,
    data: web::Data<LoginGmailRequestBody>,
) -> HttpResponse {
    let user_result = User::find_by_email(&pool, &data.email);

    match user_result {
        Ok(user) => {
            let token = UserInsensitive::new(user).jwt_tokenize();
            let login_response = LoginResponse::new(token);
            HttpResponse::Ok().json(login_response)
        }

        Err(err) => return HttpErrorResponse::bad_request(err.to_string()),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(login_gmail);
}
