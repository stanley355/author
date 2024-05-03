use crate::{db::PgPool, v2::http_error_response::HttpErrorResponse};

use super::request::FindUserQuery;

use actix_web::{get, web, HttpResponse};

#[get("")]
async fn find_user(pool: web::Data<PgPool>, query: web::Query<FindUserQuery>) -> HttpResponse {
    

    
    HttpErrorResponse::bad_request("Missing id or email query".to_string())
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(find_user);
}
