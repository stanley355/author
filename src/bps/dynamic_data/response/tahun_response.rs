use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicDataTahunResponse {
    val: u32,
    label: String,
}
