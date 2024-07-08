use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DokuNotifTransaction {
    pub _status: String,
    pub _date: String,
    pub original_request_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DokuNotifRequestBody {
    pub transaction: DokuNotifTransaction,
}
