use jsonwebkey::JsonWebKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub location: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterResponse {
    pub location: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JwksResponse {
    pub keys: Vec<JsonWebKey>,
}
