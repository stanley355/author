use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FindUserQuery {
  pub id: Option<String>,
  pub email: Option<String>
}