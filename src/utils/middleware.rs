use axum::{
    extract::{Request}, http::{self, StatusCode}, middleware::{Next}, response::{Response}
};

use crate::utils::jwt::{decode_jwt, Claims};

pub async fn check_auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(token) = authorize_current_user(auth_header).await {
        req.extensions_mut().insert(token);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str) -> Option<Claims> {
    let token = decode_jwt(auth_token.to_string());
    if let Ok(token) = token {
        return Some(token.claims);
    }
    None
}