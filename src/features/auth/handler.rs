use axum::{extract::State, http::StatusCode, middleware, routing::post, Json, Router};
use std::sync::Arc;
use crate::{container::Container, features::{auth::{model::{LoginRequest, LoginResponse}, service::AuthServiceTrait}, user::model::UserRequest}, utils::{api_response::ApiResponse, middleware::check_auth}};

pub async fn login(
    State(container): State<Arc<Container>>,
    Json(req): Json<LoginRequest>,
    // Extension(current_user): Extension<Claims>,
) -> Result<ApiResponse<LoginResponse>, ApiResponse<()>> {
    match container.auth_service.login_user(&req).await {
        Ok(token) => Ok(ApiResponse::new(StatusCode::OK, token)),
        Err(err) => Err(err),
    }
}


pub async fn register(
    State(container): State<Arc<Container>>,
    Json(mut req): Json<UserRequest>,
) -> Result<ApiResponse<String>, ApiResponse<()>> {
    match container.auth_service.register_user(&mut req).await {
        Ok(_) => Ok(ApiResponse::new(StatusCode::CREATED, "User registered successfully".to_string())),
        Err(err) => Err(err),
    }
}

pub async fn health_check(
    State(container): State<Arc<Container>>,
    Json(req): Json<String>,
) -> Result<ApiResponse<String>, ApiResponse<()>> {
    container.auth_service.health_check(&req).await
}


pub fn auth_routes() -> Router<Arc<Container>> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/health_check", post(health_check))
        // .route_layer(middleware::from_fn(check_auth))
}