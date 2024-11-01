use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::bps::dynamic_data::response::{DynamicDataTurvarResponse, DynamicDataVarResponse, DynamicDataVervarResponse, DynamicDataTahunResponse, DynamicDataTurtahunResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewDynamicDataResponse {
    status: String,
    data: Option<String>, //if this exist, means the data is empty
    var: Option<Vec<DynamicDataVarResponse>>,
    turvar: Option<Vec<DynamicDataTurvarResponse>>,
    labelvervar: Option<String>,
    vervar: Option<Vec<DynamicDataVervarResponse>>,
    tahun: Option<Vec<DynamicDataTahunResponse>>,
    turtahun: Option<Vec<DynamicDataTurtahunResponse>>,
    datacontent: Option<HashMap<String, f64>>,
}