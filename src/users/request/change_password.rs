use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct UsersChangePasswordRequest {
    pub id: String,
    pub old_password: String,
    pub new_password: String,
    pub new_password_again: String
}