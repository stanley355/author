use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebErrorResponse {
    pub status: u16,
    pub error: String,
    pub message: String,
}

impl WebErrorResponse {
    pub fn bad_request(error: diesel::result::Error, message: &str) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST.as_u16(),
            error: error.to_string(),
            message: message.to_string(),
        }
    }

    pub fn server_error(error: diesel::result::Error, message: &str) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: error.to_string(),
            message: message.to_string(),
        }
    }
}
