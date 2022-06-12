use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserModel {
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub user: UserModel,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterResponse {
    pub user: UserModel,
}
