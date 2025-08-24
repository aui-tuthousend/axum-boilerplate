use sqlx::{PgPool, Result};
use async_trait::async_trait;
use crate::features::auth::model::{LoginRequest, UserAuthResponse};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn login(&self, req : &LoginRequest) -> Result<Option<UserAuthResponse>>;
}

pub struct DbAuthRepository {
    pool: PgPool,
}

impl DbAuthRepository{
    pub fn new(pool: PgPool) -> Self {
        DbAuthRepository { pool }
    }
}

#[async_trait]
impl AuthRepository for DbAuthRepository{
    async fn login(&self, req : &LoginRequest) -> Result<Option<UserAuthResponse>> {
        let result = sqlx::query_as!(UserAuthResponse, r#"SELECT 
                id,
                password,
                role,
                is_active
                FROM users
                WHERE username = $1 and deleted_at is null LIMIT 1"#, req.username)
            .fetch_optional(&self.pool)
            .await?;
        Ok(result)
    }
}