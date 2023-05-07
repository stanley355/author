use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorRes {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginTokenRes {
    pub token: String,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct NoPasswordUser {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub phone_number: Option<String>,
}