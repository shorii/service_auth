use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebkey::JsonWebKey;
use jsonwebtoken::{encode, Algorithm, Header};
use sea_orm::entity::prelude::Uuid;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UserClaims {
    iss: String,
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

    pub fn jwt(&self, secret_key: JsonWebKey) -> String {
        let exp = Utc::now() + Duration::days(365);
        let claims = UserClaims {
            iss: "service_auth.api".to_string(),
            sub: self.username.clone(),
            exp: exp.timestamp() as usize,
        };
        encode(
            &Header {
                alg: Algorithm::ES256,
                ..Header::default()
            },
            &claims,
            &secret_key.key.to_encoding_key(),
        )
        .unwrap()
    }
}

#[async_trait]
pub trait IUserRepository {
    async fn find_by_credential(&self, username: String, password: String) -> Option<User>;
    async fn create(&self, user: User) -> Result<User, DbErr>;
}
