use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewDocumentReq {
    pub user_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FindDocumentReq{
    pub user_id: String,
    pub document_id: Option<String>
}