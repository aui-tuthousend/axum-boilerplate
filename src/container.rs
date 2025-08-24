use std::sync::Arc;
use crate::{features::{auth::{repository::DbAuthRepository, service::AuthService}, user::repository::DbUserRepository}};
use crate::utils;


#[derive(Clone)]
pub struct Container {
    pub auth_service: Arc<AuthService>,
    // pub user_repository: Arc<UserService>,
}

impl Container {
    pub async fn new() -> Self {
        let db_pool = utils::db::init().await;
        
        let user_repository = Arc::new(DbUserRepository::new(db_pool.clone()));
        
        let auth_repository = Arc::new(DbAuthRepository::new(db_pool.clone()));
        let auth_service = Arc::new(AuthService::new(auth_repository, user_repository));
        
        Container { auth_service }
    }
}