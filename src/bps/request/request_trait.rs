use std::env;
use crate::bps::request::model_enum::BpsRequestModelEnum;
use crate::bps::request::start_path_enum::BpsRequestStartPathEnum;

pub trait  BpsRequestTrait {
    fn start_path(&self) -> &BpsRequestStartPathEnum;
    fn model(&self) ->  &BpsRequestModelEnum;

    fn dynamic_param(&self) -> String;

    fn create_request_url(&self) -> String {
        let bps_api_url= env::var("BPS_API_URL").expect("Missing BPS_API_URL");
        let bps_api_key= env::var("BPS_API_KEY").expect("Missing BPS_API_KEY");

        let start_path = self.start_path().to_string().to_lowercase();
        let model = self.model().to_string().to_lowercase();
        let dynamic_param = self.dynamic_param();

        let url = format!("{bps_api_url}{start_path}/model/{model}{dynamic_param}/key/{bps_api_key}");
        return url
    }
}