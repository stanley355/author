use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginGmailRequestBody {
    pub fullname: String,
    pub email: String,
}
