use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicDataTurvarResponse {
    val: u32,
    label: String,
}