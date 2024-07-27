use actix_web::{
  dev::{forward_ready, Service, ServiceRequest, ServiceResponse},
  error::ErrorUnauthorized,
};
use std::{env, future, pin::Pin};

type PinBoxFuture<R, E> = Pin<Box<dyn future::Future<Output = Result<R, E>>>>;

#[derive(Debug)]
pub struct AuthorMiddlewareService<S> {
  pub service: S,
}

impl<S, B> AuthorMiddlewareService<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
  S::Future: 'static,
  B: 'static,
{
  fn respond_ok(
      &self,
      req: ServiceRequest,
  ) -> PinBoxFuture<ServiceResponse<B>, actix_web::Error> {
      let service_future = self.service.call(req);

      Box::pin(async move {
          let service_response = service_future.await?;
          Ok(service_response)
      })
  }

  fn respond_err(&self) -> PinBoxFuture<ServiceResponse<B>, actix_web::Error> {
      Box::pin(async { Err(ErrorUnauthorized("".to_string())) })
  }
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
      if req.path() == "/v1/subscriptions/doku/notification/" ||  req.path().contains("/v1/files") {
          return self.respond_ok(req);
      }

      if let Some(authorization) = req.headers().get("authorization") {
          let token = authorization.to_str().unwrap();
          let bearer_token = env::var("BEARER_TOKEN").expect("BEARER_TOKEN must be set");

          return match token == bearer_token {
              true => self.respond_ok(req),
              false => self.respond_err(),
          };
      }

      return self.respond_err();
  }
}
