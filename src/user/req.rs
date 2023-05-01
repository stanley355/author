use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct GmailLoginReq {
    pub fullname: String,
    pub email: String,
}
