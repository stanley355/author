use actix_web::dev::ServiceRequest;

use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};

pub async fn bearer_validator(
    req: ServiceRequest,
    bearer_auth: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    match bearer_auth.token() {
        "abc" => Ok(req),
        _ => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default();

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}
