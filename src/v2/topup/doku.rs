use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DokuNotifTransaction {
    #[allow(dead_code)]
    pub status: String,
    #[allow(dead_code)]
    pub date: String,
    pub original_request_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DokuNotifRequestBody {
    pub transaction: DokuNotifTransaction,
}
