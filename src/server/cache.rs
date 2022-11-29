use axum::{
    body::Body,
    http::{HeaderValue, Request},
    response::Response,
};
use futures::future::BoxFuture;
use hyper::header::CACHE_CONTROL;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Debug, Clone, Default)]
pub struct Cache(u32);

impl Cache {
    pub fn new(days: u32) -> Self {
        Cache(days * 24 * 60 * 60)
    }
}

impl<S> Layer<S> for Cache {
    type Service = CacheMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CacheMiddleware { inner, age: self.0 }
    }
}

#[derive(Clone)]
pub struct CacheMiddleware<S> {
    inner: S,
    age: u32,
}

impl<S> Service<Request<Body>> for CacheMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let future = self.inner.call(request);
        let no_cache = HeaderValue::from_static("no-cache");
        let cache = if cfg!(debug_assertions) {
            no_cache
        } else {
            HeaderValue::from_str(format!("max-age={}", self.age).as_str()).unwrap_or(no_cache)
        };

        Box::pin(async move {
            let mut response: Response = future.await?;
            let headers = response.headers_mut();
            headers.append(CACHE_CONTROL, cache);
            Ok(response)
        })
    }
}

