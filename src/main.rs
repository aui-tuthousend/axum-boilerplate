use axum::response::Html;
use axum::routing::get;
use axum::{Router};
use axum_boilerplate::container::Container;
use axum_boilerplate::features::auth::handler::auth_routes;
use dotenv::dotenv;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    unsafe { std::env::set_var("RUST_LOG", "debug,axum=debug,axum_web=debug"); }
    env_logger::init();
    let container = Arc::new(Container::new().await);

    let app = Router::new()
        .route("/", get(|| async { Html("Supanigaaaa!")}))
        .nest("/api/auth", auth_routes(container.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
