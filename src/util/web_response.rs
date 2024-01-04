use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: T
}
