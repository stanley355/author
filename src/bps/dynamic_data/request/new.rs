use serde::Deserialize;
use crate::bps::request::{BpsRequestModelEnum, BpsRequestStartPathEnum};
use crate::bps::request_trait::BpsRequestTrait;

#[derive(Debug, Deserialize)]
pub(in crate::bps::dynamic_data) struct NewDynamicDataRequestParam {
    domain: String,
    var: u32,
    turvar: Option<String>,
    vervar: Option<String>,
    th: Option<String>,
    turth: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(in crate::bps::dynamic_data) struct NewDynamicDataRequest {
    start_path: BpsRequestStartPathEnum,
    model: BpsRequestModelEnum,
    param: NewDynamicDataRequestParam
}

impl BpsRequestTrait for NewDynamicDataRequest {
    fn start_path(&self) -> &BpsRequestStartPathEnum {
        &self.start_path
    }

    fn model(&self) -> &BpsRequestModelEnum {
        &self.model
    }

    fn dynamic_param(&self) -> String {
        let param= &self.param;
        let domain = &param.domain;
        let var = &param.var;
        let mut param_path = format!("/domain/{domain}/var/{var}");

        if let Some(turvar) = &param.turvar {
            let turvar_param = format!("/turvar/{turvar}");
            param_path.push_str(&turvar_param);
        }

        if let Some(vervar) = &param.vervar {
            let vervar_param = format!("/vervar/{vervar}");
            param_path.push_str(&vervar_param);
        }

        if let Some(th) = &param.vervar {
            let th_param = format!("/th/{th}");
            param_path.push_str(&th_param);
        }

        if let Some(turth) = &param.turth{
            let turth_param = format!("/turth/{turth}");
            param_path.push_str(&turth_param);
        }

        return param_path;
    }
}

impl NewDynamicDataRequest {
    pub fn new(model: BpsRequestModelEnum, param: NewDynamicDataRequestParam) -> Self {
        Self {
            start_path: BpsRequestStartPathEnum::List,
            model,
            param
        }
    }
}