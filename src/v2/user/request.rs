use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginGmailRequestBody {
  pub email: String
}