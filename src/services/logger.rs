use std::{task::{Context, Poll}, future::Future, pin::Pin};
use axum::http::{Request, Response};
use tower::{Layer, Service};
use std::time::Instant;

#[derive(Clone)]
pub struct LoggingLayer;

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggingService { inner }
    }
}

#[derive(Clone)]
pub struct LoggingService<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for LoggingService<S>
where
    S: Service<Request<ReqBody>, Response = Response<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: std::fmt::Display,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let method = req.method().clone();
        let uri = req.uri().clone();
        let start = Instant::now();

        let fut = self.inner.call(req);

        Box::pin(async move {
            let res = fut.await;
            let duration = start.elapsed();
            match &res {
                Ok(resp) => {
                    println!(
                        "[{} {}] => {} in {:?}",
                        method,
                        uri,
                        resp.status(),
                        duration
                    );
                }
                Err(err) => {
                    eprintln!("[{} {}] failed: {}", method, uri, err);
                }
            }
            res
        })
    }
}
