use diesel::Queryable;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct CreateDocumentReq {
    pub user_id: String,
    pub name: String,
    pub doc_type: String,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct FindDocumentsReq{
    pub id: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct DeleteDocumentReq{
    pub id: String,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct UpdateDocumentReq{
    pub id: String,
    pub name: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DocType {
    Translate,
    Checkbot,
}

impl fmt::Display for DocType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DocType::Translate => write!(f, "translate"),
            DocType::Checkbot => write!(f, "checkbot"),
        }
    }
}
