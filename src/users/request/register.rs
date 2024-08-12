use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct UsersRegisterRequest {
    pub fullname: String,
    pub email:String,
    pub password: String,
    pub password_again: String
}