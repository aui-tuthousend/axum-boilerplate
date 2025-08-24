use std::sync::Arc;

use async_trait::async_trait;
use axum::http::StatusCode;

use crate::{features::user::{model::UserResponse, repository::UserRepository}, utils::api_response::ApiResponse};

#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn get_all_user(&self) -> Result<Vec<UserResponse>, ApiResponse<()>>;
}

pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        UserService { repository }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn get_all_user(&self) -> Result<Vec<UserResponse>, ApiResponse<()>> {
        let user = self.repository.get_all_user().await.map_err(|e| {
            ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                Some(e.to_string()),
            )
        })?;

        Ok(user)
    }
}


