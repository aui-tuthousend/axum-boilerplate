use axum::{
    http::{StatusCode},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: i64,
    pub role: String,
    pub exp: usize,
}

pub fn encode_jwt(user_id: i64, role: String) -> String {
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let claim = Claims { user_id, exp, role };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(env::var("JWT_SECRET_KEY").unwrap().as_ref())).unwrap()
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = env::var("JWT_SECRET_KEY").unwrap();
    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}