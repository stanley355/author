use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DokuNotifTransaction {
    pub status: String,
    pub date: String,
    pub original_request_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DokuNotifRequestBody {
    pub transaction: DokuNotifTransaction,
}
