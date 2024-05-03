use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FindUserQuery {
  id: Option<String>,
  email: Option<String>
}