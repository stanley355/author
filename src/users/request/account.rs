use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UsersAccountRequest {
    pub id: String
}