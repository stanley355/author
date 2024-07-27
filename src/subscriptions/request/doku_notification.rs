use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct DokuNotificationRequest {
    pub transaction: DokuNotificationRequestTransactionParameter,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct DokuNotificationRequestTransactionParameter {
    status: String,
    date: String,
    pub(crate) original_request_id: String,
}
