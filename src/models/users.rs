use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{auth::UserResponse, database::Database, error::ApiResult};

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

pub trait UserDataLayer {
    async fn get_user<'a>(&'a self, id: &'a Uuid) -> ApiResult<Option<User>>;
    async fn upsert_user<'a>(&'a self, response: &'a UserResponse) -> ApiResult<User>;
    async fn delete_user<'a>(&'a self, id: &'a Uuid) -> ApiResult<()>;
}

impl Database {
    #[allow(dead_code)]
    async fn create_user<'a>(&'a self, name: &'a str) -> ApiResult<Uuid> {
        let id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();
        sqlx::query(r#"INSERT INTO user (id, name, created_at) VALUES ($1, $2, $3) RETURNING id, name, created_at, updated_at"#)
            .bind(id)
            .bind(name)
            .bind(now)
            .execute(&self.db)
            .await?;
        Ok(id)
    }
}

impl UserDataLayer for Database {
    async fn get_user<'a>(&'a self, id: &'a Uuid) -> ApiResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"SELECT id, name, created_at, updated_at FROM user WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?;
        Ok(user)
    }

    async fn upsert_user<'a>(&'a self, response: &'a UserResponse) -> ApiResult<User> {
        let mut trx = self.db.begin().await?;

        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT 
                user.id, name, created_at, updated_at 
            FROM 
                user 
            LEFT JOIN 
                account 
            WHERE 
                account.id = ?
            "#,
        )
        .bind(&response.sub)
        .fetch_optional(&mut *trx)
        .await?;

        if let Some(user) = user {
            trx.commit().await?;
            return Ok(user);
        }

        let id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO user (
                id,
                name,
                created_at
            ) VALUES 
                ($1, $2, $3) 
            RETURNING 
                id,
                name,
                created_at,
                updated_at
            "#,
        )
        .bind(id)
        .bind(&response.name)
        .bind(now)
        .fetch_one(&mut *trx)
        .await?;

        sqlx::query(r#"INSERT INTO account (id, provider, user_id) VALUES ($1, $2, $3)"#)
            .bind(&response.sub)
            .bind("google")
            .bind(&user.id)
            .execute(&mut *trx)
            .await?;

        trx.commit().await?;
        Ok(user)
    }

    async fn delete_user<'a>(&'a self, id: &'a Uuid) -> ApiResult<()> {
        sqlx::query(r#"DELETE FROM user WHERE id = ?"#)
            .bind(id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}
