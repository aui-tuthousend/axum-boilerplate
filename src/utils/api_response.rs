use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(code: StatusCode, data: T) -> Self {
        Self {
            code: code.as_u16(),
            message: Some("Success".to_string()),
            data: Some(data),
        }
    }

    pub fn error(code: StatusCode, message: Option<String>) -> Self {
        Self {
            code: code.as_u16(),
            message,
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.code).unwrap(), Json(self)).into_response()
    }
}
