use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::entity::prelude::Uuid;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UserClaims {
    sub: String,
    exp: usize,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        User {
            id: Uuid::new_v4(),
            username,
            password,
        }
    }

    pub fn jwt(&self) -> String {
        let exp = Utc::now() + Duration::days(365);
        let claims = UserClaims {
            sub: self.username.clone(),
            exp: exp.timestamp() as usize,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("SECRET_KEY".as_bytes()),
        )
        .unwrap()
    }
}

#[async_trait]
pub trait IUserRepository {
    async fn find_by_credential(&self, username: String, password: String) -> Option<User>;
    async fn create(&self, user: User) -> Result<User, DbErr>;
}
