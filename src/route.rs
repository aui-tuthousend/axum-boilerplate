use std::sync::Arc;
use axum::{Router};

use crate::container::Container;
use crate::features::auth::handler::auth_routes;


pub fn public_route() -> Router<Arc<Container>> {
    Router::new()
        .nest("/api/auth", auth_routes())
}