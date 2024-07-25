use super::author_middleware_service::AuthorMiddlewareService;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use std::future::{ready, Ready};

#[derive(Debug)]
pub struct AuthorMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthorMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthorMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorMiddlewareService { service }))
    }
}
