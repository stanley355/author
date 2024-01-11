use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebErrorResponse {
    pub status: u16,
    pub error: String,
    pub message: String,
}
