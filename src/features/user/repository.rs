use sqlx::{PgPool, Result};
use async_trait::async_trait;
use crate::features::user::model::{User, UserRequest, UserResponse, UserUpdateRequest};
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_all_user(&self) -> Result<Vec<UserResponse>>;
    async fn get_user_by_username_or_email(&self, username_or_email: &str) -> Result<Option<User>>;
    async fn create_user(&self, user: &UserRequest) -> Result<()>;
    async fn update_user(&self, user: &UserUpdateRequest) -> Result<()>;
    async fn delete_user(&self, uuid: &Uuid) -> Result<()>;
}

pub struct DbUserRepository {
    pool: PgPool,
}

impl DbUserRepository {
    pub fn new(pool: PgPool) -> Self {
        DbUserRepository { pool }
    }
}

#[async_trait]
impl UserRepository for DbUserRepository{
    async fn get_all_user(&self) -> Result<Vec<UserResponse>> {
        let users = sqlx::query_as!(UserResponse, r#"
            SELECT 
                u.uuid::text,
                u.email,
                u.username,
                u.role
            FROM users u 
            where u.deleted_at is null"#)
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }
    async fn get_user_by_username_or_email(&self, username_or_email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(User, r#"
            SELECT * 
            FROM users 
            WHERE username = $1 or email = $1 and deleted_at is null
        "#, username_or_email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }
    async fn create_user(&self, user: &UserRequest) -> Result<()> {
        sqlx::query!(r#"
            INSERT INTO users (email, username, password, role) 
            VALUES ($1, $2, $3, $4)
        "#, user.email, user.username, user.password, user.role)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn update_user(&self, user: &UserUpdateRequest) -> Result<()> {
        sqlx::query!(r#"
            UPDATE users SET email = $2, username = $3, updated_at = now(), role = $4 
            WHERE uuid::text = $1
        "#, user.uuid, user.email, user.username, user.role)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn delete_user(&self, uuid: &Uuid) -> Result<()> {
        sqlx::query!(r#"UPDATE users SET deleted_at = now() WHERE uuid = $1"#, uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}