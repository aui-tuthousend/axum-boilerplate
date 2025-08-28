use axum::http::{Request, Response, StatusCode};
use tokio::sync::Semaphore;
use tower::{Layer, Service};
use std::{future::Future, pin::Pin, sync::Arc, task::{Context, Poll}};


#[derive(Clone)]
pub struct RateLimitLayer {
    permits: Arc<Semaphore>,
}

impl RateLimitLayer {
    pub fn new(max: usize) -> Self {
        Self {
            permits: Arc::new(Semaphore::new(max)),
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            permits: self.permits.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    permits: Arc<Semaphore>,
}

impl<S, ReqBody> Service<Request<ReqBody>> for RateLimitService<S>
where
    S: Service<Request<ReqBody>, Response = Response<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: std::fmt::Display,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
    let permits = self.permits.clone();
    let mut inner = self.inner.clone(); 

    let fut = async move {
        let permit = permits.try_acquire_owned();

        match permit {
            Ok(_p) => {
                inner.call(req).await
            }
            Err(_) => {
                let resp = Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS)
                    .body(axum::body::Body::from("Rate limit exceeded"))
                    .unwrap();
                Ok(resp)
            }
        }
    };

    Box::pin(fut)
}

}
