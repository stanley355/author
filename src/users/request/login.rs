use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct UsersLoginRequest {
    pub email: String,
    pub password: String,
}

