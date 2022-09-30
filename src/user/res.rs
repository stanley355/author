use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginTokenRes {
    pub token: String,
}

impl LoginTokenRes {
    pub fn new(token: String) -> Self {
        LoginTokenRes { token }
    }
}
