use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct UsersAccountRequest {
    pub id: String
}