use jsonwebkey::JsonWebKey;
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub location: String,
    pub secret_key: JsonWebKey,
}
