use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct GetUserParam {
    pub email: String
}


#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct GetAccountParam {
    pub id: String
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

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct ReduceBalanceReq{
    pub user_id: String,
    pub reduce_amount: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserIdReq{
    pub user_id: String
}