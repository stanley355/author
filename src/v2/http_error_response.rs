use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HttpErrorResponse {
    status: u16,
    status_text: String,
}

impl HttpErrorResponse {
    pub fn new(status: u16, status_text: String) -> Self {
        Self {
            status,
            status_text,
        }
    }

    pub fn internal_server_error(error_message: String) -> HttpResponse {
        let error = Self::new(500, error_message);
        HttpResponse::InternalServerError().json(error)
    }

    pub fn payment_required() -> HttpResponse {
        let error = Self::new(402, "Payment Required".to_string());
        HttpResponse::PaymentRequired().json(error)
    }
}
