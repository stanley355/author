use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct GetUserParam {
    pub email: Option<String>
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct RegisterReq {
    pub fullname: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct LoginReq{
    pub email: String,
    pub password: String,
}


#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct GmailLoginReq {
    pub fullname: String,
    pub email: String,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct IncreaseBalanceReq{
    pub user_id: String,
    pub increase_amount: f64,
}
