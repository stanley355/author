use super::model::User;
use super::request::FindUserQuery;
use super::user_insensitive::UserInsensitive;
use crate::{db::PgPool, v2::http_error_response::HttpErrorResponse};

use actix_web::{get, web, HttpResponse};

#[get("")]
async fn find_user(pool: web::Data<PgPool>, query: web::Query<FindUserQuery>) -> HttpResponse {

    if let Some(id) = &query.id {
        let user_result = User::find(&pool, id);

        match user_result {
            Ok(user) => {
                let user_insensitive = UserInsensitive::new(user);
                return HttpResponse::Ok().json(user_insensitive);
            }
            Err(_) => return HttpErrorResponse::bad_request("User not found".to_string()),
        }
    }

    if let Some(email) = &query.email {
        let user_result = User::find_by_email(&pool, email);

        match user_result {
            Ok(user) => {
                let user_insensitive = UserInsensitive::new(user);
                return HttpResponse::Ok().json(user_insensitive);
            }
            Err(_) => return HttpErrorResponse::bad_request("User not found".to_string()),
        }
    }

    HttpErrorResponse::bad_request("Missing id or email query".to_string())
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(find_user);
}
