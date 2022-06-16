use crate::domain::{IUserRepository, User};
use crate::infra::db::user;
use async_trait::async_trait;
use sea_orm::query::Statement;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, DbErr, FromQueryResult};

pub struct UserRepository {
    conn: DatabaseConnection,
}

impl UserRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn find_by_credential(&self, username: String, password: String) -> Option<User> {
        let model = user::Model::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            SELECT
                id,
                username,
                password
            FROM
                "auth"."user"
            WHERE
                username = $1
                AND password = crypt($2, password)
            "#,
            vec![username.into(), password.into()],
        ))
        .one(&self.conn)
        .await
        .expect("db error occurred");
        match model {
            Some(m) => Some(User {
                id: m.id,
                username: m.username,
                password: m.password,
            }),
            None => None,
        }
    }

    async fn create(&self, user: User) -> Result<User, DbErr> {
        self.conn.execute(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"INSERT INTO "auth"."user" (username, password) VALUES ($1, crypt($2, gen_salt('bf')))"#,
            {
                let user = user.clone();
                vec![user.username.into(), user.password.into()]
            }
        ))
        .await?;
        Ok(user)
    }
}
