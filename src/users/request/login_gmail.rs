use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct UsersLoginGmailRequest {
    pub fullname: String,
    pub email: String,
}