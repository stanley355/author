use actix_web::dev::ServiceRequest;
use std::env;

// pub async fn bearer_validator(
//     req: ServiceRequest,
//     bearer_auth: BearerAuth,
// ) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
//     let bearer_token = bearer_auth.token();
//     let token = &env::var("BEARER_TOKEN").unwrap();
//     let is_token_valid = bearer_token == token;
//     let is_doku_notification = req.path() == "/v1/topups/doku/notification/";

//     if is_doku_notification || is_token_valid {
//         return Ok(req);
//     } else {
//         Err((actix_web::error::ErrorUnauthorized("Forbidden"), req))
//     }
// }
