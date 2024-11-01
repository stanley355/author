use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicDataTurtahunResponse {
    val: u32,
    label: String,
}
