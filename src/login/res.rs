use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRes {
    pub token: String,
}

impl LoginRes {
    pub fn new(token: String) -> Self {
        LoginRes { token }
    }
}
