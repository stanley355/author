use crate::schema::users;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct LoginReq {
    pub fullname: String,
    pub email: String,
    pub password: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUserReq {
    pub fullname: String,
    pub email: String,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub has_channel: Option<bool>,
}
