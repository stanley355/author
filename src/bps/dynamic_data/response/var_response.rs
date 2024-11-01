use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicDataVarResponse {
    val: u32,
    label: String,
    unit: String,
    subj: String,
    def: String,
    decimal: String,
    note: String
}