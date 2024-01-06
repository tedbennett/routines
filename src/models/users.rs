use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::database::Database;

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

pub trait UserDataLayer {
    async fn get_user<'a>(&'a self, id: &'a Uuid) -> Result<Option<User>, ApiError>;
    async fn create_user<'a>(&'a self, name: &'a str) -> Result<Uuid, ApiError>;
    async fn delete_user<'a>(&'a self, id: &'a Uuid) -> Result<(), ApiError>;
}

impl UserDataLayer for Database {
    async fn get_user<'a>(&'a self, id: &'a Uuid) -> Result<Option<User>, ApiError> {
        let user = sqlx::query_as::<_, User>(
            r#"SELECT id as "id: uuid::Uuid", name, created_at, updated_at FROM user WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?;
        Ok(user)
    }

    async fn create_user<'a>(&'a self, name: &'a str) -> Result<Uuid, ApiError> {
        let id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();
        sqlx::query(r#"INSERT INTO user (id, name, created_at) VALUES ($1, $2, $3)"#)
            .bind(id)
            .bind(name)
            .bind(now)
            .execute(&self.db)
            .await?;
        Ok(id)
    }

    async fn delete_user<'a>(&'a self, id: &'a Uuid) -> Result<(), ApiError> {
        sqlx::query(r#"DELETE FROM user WHERE id = ?"#)
            .bind(id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}

pub enum ApiError {
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError(value)
    }
}
