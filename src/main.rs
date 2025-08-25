
use axum_boilerplate::container::Container;
use axum_boilerplate::route::public_route;
use dotenv::dotenv;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    unsafe { std::env::set_var("RUST_LOG", "debug,axum=debug,axum_web=debug"); }
    env_logger::init();

    let container = Arc::new(Container::new().await);
    let app = public_route().with_state(container.clone());
        

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
