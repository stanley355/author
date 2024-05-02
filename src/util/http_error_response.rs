use actix_web::{http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpErrorResponse {
    pub status: u16,
    pub error: String,
    pub message: String,
}

impl HttpErrorResponse {
    pub fn new(status: Option<u16>, error: String, message: &str) -> Self {
        HttpErrorResponse {
            status: match status {
                Some(status_code) => status_code,
                None => 500,
            },
            error,
            message: message.to_string(),
        }
    }

    pub fn response(self) -> HttpResponse {
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(self)
    }
}
