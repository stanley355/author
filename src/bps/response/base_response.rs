use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BpsBaseResponse<S> {
    status: String,
    data: Option<S>,
    var: Option<S>
}