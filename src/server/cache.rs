use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderValue, CACHE_CONTROL},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::ready;
use std::future::Ready;

pub struct CacheInterceptor(u32);

impl CacheInterceptor {
    pub fn new(days: u32) -> Self {
        CacheInterceptor(days * 24 * 60 * 60)
    }
}

impl Default for CacheInterceptor {
    fn default() -> Self {
        CacheInterceptor::new(7)
    }
}

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
        ready(Ok(CacheInterceptorMiddleware {
            service,
            age: self.0,
        }))
    }
}

pub struct CacheInterceptorMiddleware<S> {
    age: u32,
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
        let no_cache = HeaderValue::from_static("no-cache");
        let cache = if cfg!(debug_assertions) {
            no_cache
        } else {
            HeaderValue::from_str(format!("max-age={}", self.age).as_str()).unwrap_or(no_cache)
        };

        Box::pin(async move {
            let mut res = fut.await?;
            let headers = res.headers_mut();
            headers.append(CACHE_CONTROL, cache);
            Ok(res)
        })
    }
}
