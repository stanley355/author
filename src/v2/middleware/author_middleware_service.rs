use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse};
use std::{future, pin::Pin};

// type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;
type PinBoxFuture<R, E> = Pin<Box<dyn future::Future<Output = Result<R, E>>>>;

pub struct AuthorMiddlewareService<S> {
    pub service: S,
}

impl<S, B> Service<ServiceRequest> for AuthorMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = PinBoxFuture<Self::Response, Self::Error>;

    // This service is ready when its next service is ready
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service_future = self.service.call(req);

        Box::pin(async move {
            let service_response = service_future.await?;
            Ok(service_response)
        })
    }
}
