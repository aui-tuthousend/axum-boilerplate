use axum::{extract::State, routing::{post}, Json, Router};
use std::sync::Arc;
use crate::{container::Container, features::{auth::{model::{LoginRequest, LoginResponse}, service::AuthServiceTrait}, user::model::{UserRequest}}, utils::api_response::ApiResponse};

pub async fn login(
    State(container): State<Arc<Container>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Json<String>> {
    match container.auth_service.login_user(&req).await {
        Ok(token) => Ok(Json(token)),
        Err(err) => Err(Json((err.unwrap()).to_string())),
    }
}

pub async fn register(
    State(container): State<Arc<Container>>,
    Json(mut req): Json<UserRequest>,
) -> Result<Json<String>, ApiResponse<()>> {
    match container.auth_service.register_user(&mut req).await {
        Ok(_) => Ok(Json("User registered successfully".to_string())),
        Err(err) => Err(err),
    }
}

pub fn auth_routes(container: Arc<Container>) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(container)
}