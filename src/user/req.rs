use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct GmailLoginReq {
    pub fullname: String,
    pub email: String,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct TokenPayload {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub phone_number: Option<String>,
}