use serde::{Deserialize, Serialize};

// TODO: Remove this after refac all error
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorRes {
    pub error: String,
    pub message: String,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserLoginRes {
    pub token: String,
}
