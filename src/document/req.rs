use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewDocumentReq {
    pub user_id: String,
    pub name: String,
}