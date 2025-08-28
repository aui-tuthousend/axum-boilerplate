use axum_boilerplate::container::Container;
use axum_boilerplate::route::public_route;
// use axum_boilerplate::services::buffer::BufferLayer;
use axum_boilerplate::services::concurrency_limit::{ConcurrencyLimitLayer};
use axum_boilerplate::services::logger::LoggingLayer;
use axum_boilerplate::services::rate_limit::RateLimitLayer;

use tower::buffer::BufferLayer;
// use tower::limit::ConcurrencyLimitLayer;

use dotenv::dotenv;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    dotenv().ok();
    unsafe { std::env::set_var("RUST_LOG", "debug,axum=debug,axum_web=debug"); }
    env_logger::init();

    let container = Arc::new(Container::new().await);

    let app = public_route()
        .with_state(container.clone())
        .layer(LoggingLayer)
        .layer(RateLimitLayer::new(10))
        .layer(ConcurrencyLimitLayer::new(10));
    // .layer(BufferLayer::new(100))

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
