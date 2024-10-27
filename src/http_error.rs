use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HttpError {
    status: u16,
    status_text: String,
}

impl HttpError {
    pub fn new(status: u16, status_text: String) -> Self {
        Self {
            status,
            status_text,
        }
    }

    pub fn bad_request(text: &str) -> HttpResponse {
        let error = Self::new(400, text.to_string());
        HttpResponse::BadRequest().json(error)
    }

    pub fn internal_server_error(text: &str) -> HttpResponse {
        let error = Self::new(500, text.to_string());
        HttpResponse::InternalServerError().json(error)
    }
}
