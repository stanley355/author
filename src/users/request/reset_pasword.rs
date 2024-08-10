use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct UsersResetPasswordRequest {
    pub id: String,
    pub email: String,
    pub new_password:String,
    pub admin_password: String
}