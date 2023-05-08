use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};
use std::env;

pub async fn bearer_validator(
    req: ServiceRequest,
    bearer_auth: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let req_token = bearer_auth.token();
    let token = &env::var("BEARER_TOKEN").unwrap_or("no_token_env".to_string());

    if req_token == token {
        Ok(req)
    } else {
        let config = req
            .app_data::<bearer::Config>()
            .cloned()
            .unwrap_or_default();

        Err((AuthenticationError::from(config).into(), req))
    }
}
