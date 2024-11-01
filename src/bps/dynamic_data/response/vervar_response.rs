use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicDataVervarResponse {
    val: u32,
    label: String,
}
