use super::model::User;
use super::request::AccountPageDataRequestQuery;
use super::response::LoginResponse;
use super::user_insensitive::UserInsensitive;
use crate::v2::user::request::LoginGmailRequestBody;
use crate::{db::PgPool, v2::http_error_response::HttpErrorResponse};
use actix_web::{get, post, web, HttpResponse};

#[post("/login/gmail/")]
async fn login_gmail(
    pool: web::Data<PgPool>,
    body: web::Json<LoginGmailRequestBody>,
) -> HttpResponse {
    let is_email_valid = User::check_email_valid(&body.email);
    if is_email_valid == false {
        return HttpErrorResponse::bad_request("Invalid Email".to_string());
    }

    let user_result = User::find_by_email(&pool, &body.email);
    if let Ok(user) = user_result {
        let token = UserInsensitive::new(user).jwt_tokenize();
        let login_response = LoginResponse::new(token);
        return HttpResponse::Ok().json(login_response);
    }

    let new_user_result = User::insert_one_by_gmail(&pool, &body);
    match new_user_result {
        Ok(new_user) => {
            let token = UserInsensitive::new(new_user).jwt_tokenize();
            let login_response = LoginResponse::new(token);
            HttpResponse::Ok().json(login_response)
        }
        Err(err) => return HttpErrorResponse::bad_request(err.to_string()),
    }
}

#[get("/account")]
async fn get_account_page_data(
    pool: web::Data<PgPool>,
    query: web::Query<AccountPageDataRequestQuery>,
) -> HttpResponse {
    let account_page_data = User::get_account_page_data(&pool, &query.id);

    HttpResponse::Ok().json(account_page_data)
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(login_gmail).service(get_account_page_data);
}
