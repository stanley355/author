use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
}

impl LoginResponse {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
