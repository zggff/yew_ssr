use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderValue, CACHE_CONTROL},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::ready;
use std::future::Ready;

pub struct CacheInterceptor;

impl<S, B> Transform<S, ServiceRequest> for CacheInterceptor
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CacheInterceptorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CacheInterceptorMiddleware { service }))
    }
}

pub struct CacheInterceptorMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CacheInterceptorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            let headers = res.headers_mut();
            headers.append(CACHE_CONTROL, HeaderValue::from_static("max-age=31536000"));
            Ok(res)
        })
    }
}
