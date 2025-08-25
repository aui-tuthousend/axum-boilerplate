use std::sync::Arc;

use async_trait::async_trait;
use crate::{features::{auth::{model::{LoginRequest, LoginResponse}, repository::AuthRepository}, user::{model::UserRequest, repository::UserRepository}}, utils::{api_response::ApiResponse, hash_pass::{hash_password, verify_password}, jwt::encode_jwt}};
use axum::http::StatusCode;
use validator::Validate;

#[async_trait]
pub trait AuthServiceTrait: Send + Sync {
    async fn login_user(&self, req : &LoginRequest) -> Result<LoginResponse, ApiResponse<()>>;
    async fn register_user(&self, req: &mut UserRequest) -> Result<(), ApiResponse<()>>;
    async fn health_check(&self, req: &String) -> Result<ApiResponse<String>, ApiResponse<()>>;
}

pub struct AuthService {
    repository: Arc<dyn AuthRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl AuthService {
    pub fn new(repository: Arc<dyn AuthRepository>, user_repository: Arc<dyn UserRepository>) -> Self {
        AuthService { repository, user_repository }
    }
}


#[async_trait]
impl AuthServiceTrait for AuthService {
    async fn login_user(&self, req: &LoginRequest) -> Result<LoginResponse, ApiResponse<()>> {
        req.validate().map_err(|e| {
            ApiResponse::error(
                StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                Some(e.to_string()),
            )
        })?;
    
        let user = self.repository.login(req).await.map_err(|e| {
            ApiResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                Some(e.to_string()),
            )
        })?;
    
        let user_verify = user.unwrap();
    
        let verify = verify_password(&req.password, &user_verify.password);
        if !verify.unwrap() {
            return Err(ApiResponse::error(
                StatusCode::UNAUTHORIZED.as_u16(),
                Some("Invalid password".to_string()),
            ));
        }
    
        let token = encode_jwt(user_verify.id, user_verify.role);
        Ok(LoginResponse { token })
    }
    
    
    
    async fn register_user(&self, req: &mut UserRequest) -> Result<(), ApiResponse<()>> {
        req.validate().map_err(|e| {
            ApiResponse::error(
                StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                Some(e.to_string()),
            )
        })?;

        if req.password.len() < 8 {
            return Err(ApiResponse::error(
                StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                Some("Password must be at least 8 characters long".to_string()),
            ));
        }

        let hash_password = hash_password(&req.password);

        let user = UserRequest {
            email: req.email.clone(),
            username: req.username.clone(),
            password: hash_password.unwrap(),
            role: req.role.clone(),
        };

        self.user_repository.create_user(&user).await.map_err(|e| {
            ApiResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                Some(e.to_string()),
            )
        })?;

        Ok(())
    }

    async fn health_check(&self, req: &String) -> Result<ApiResponse<String>, ApiResponse<()>> {
        Ok(ApiResponse::new(req.clone()))
    }
}