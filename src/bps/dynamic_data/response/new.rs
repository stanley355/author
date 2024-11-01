use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::bps::dynamic_data::response::{DynamicDataTurvarResponse, DynamicDataVarResponse, DynamicDataVervarResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewDynamicDataResponse {
    status: String,
    data: Option<String>, //if this exist, means the data is empty
    var: Vec<DynamicDataVarResponse>,
    turvar: Vec<DynamicDataTurvarResponse>,
    labelvervar: String,
    vervar: Vec<DynamicDataVervarResponse>,
    datacontent: HashMap<String, f64>
}