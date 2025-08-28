use std::task::{Context, Poll};
use tower::{Layer, Service};
use axum::{
    body::Body, 
    http::{Request, Response, StatusCode}
};
use futures_util::future::BoxFuture;
use tokio::sync::Semaphore;
use std::sync::Arc;

#[derive(Clone)]
pub struct ConcurrencyLimitLayer {
    max_concurrent: usize,
}

impl ConcurrencyLimitLayer {
    pub fn new(max_concurrent: usize) -> Self {
        Self { max_concurrent }
    }
}

impl<S> Layer<S> for ConcurrencyLimitLayer {
    type Service = ConcurrencyLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ConcurrencyLimitService::new(inner, self.max_concurrent)
    }
}

// The Service that actually handles the concurrency limiting
#[derive(Clone)]
pub struct ConcurrencyLimitService<S> {
    inner: S,
    semaphore: Arc<Semaphore>,
}

impl<S> ConcurrencyLimitService<S> {
    pub fn new(inner: S, max_concurrent: usize) -> Self {
        Self {
            inner,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }
}

impl<S, ReqBody> Service<Request<ReqBody>> for ConcurrencyLimitService<S>
where
    S: Service<Request<ReqBody>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let semaphore = self.semaphore.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Try to acquire permit (non-blocking)
            match semaphore.try_acquire_owned() {
                Ok(_permit) => {
                    // Permit is automatically released when dropped
                    inner.call(req).await
                }
                Err(_) => {
                    // No permits available
                    Ok(Response::builder()
                        .status(StatusCode::TOO_MANY_REQUESTS)
                        .body(Body::from("Too many concurrent requests"))
                        .unwrap())
                }
            }
        })
    }
}