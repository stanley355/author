use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorRes {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginTokenRes {
    pub token: String,
}