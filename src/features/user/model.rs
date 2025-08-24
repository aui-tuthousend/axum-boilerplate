use serde::{Deserialize, Serialize};
use sqlx::{Type};
use chrono::NaiveDateTime;
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User{
    pub id: i64,
    pub uuid: Option<Uuid>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub is_active: Option<bool>,
    pub role: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserRequest{
    #[validate(email(message = "Invalid email format"), length(min = 1, message = "Email is required"))]
    pub email: String,
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(length(min = 1, message = "Role is required"))]
    pub role: String
}

#[derive(Debug, Serialize, Type)]
pub struct UserResponse{
    pub uuid: Option<String>,
    pub email: String,
    pub username: String,
    pub role: String,
}

#[derive(Deserialize, Validate)]
pub struct UserUpdateRequest{
    #[validate(length(min = 1, message = "UUID is required"))]
    pub uuid: String,
    #[validate(length(min = 1, message = "Email is required"))]
    pub email: String,
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    #[validate(length(min = 1, message = "Role is required"))]
    pub role: String,
}


