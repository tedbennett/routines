use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{database::Database, error::ApiResult};

#[derive(FromRow)]
pub struct Routine {
    pub id: Uuid,
    pub title: String,
    pub color: String,
    pub user_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

pub trait RoutineDataLayer {
    async fn get_routine<'a>(&'a self, id: &'a Uuid) -> ApiResult<Option<Routine>>;
    async fn get_routines<'a>(&'a self, user_id: &'a Uuid) -> ApiResult<Vec<Routine>>;
    async fn create_routine<'a>(
        &'a self,
        title: &'a str,
        color: &'a str,
        user_id: &'a Uuid,
    ) -> ApiResult<Uuid>;
    async fn delete_routine<'a>(&'a self, id: &'a Uuid) -> ApiResult<()>;
}

impl RoutineDataLayer for Database {
    async fn get_routine<'a>(&'a self, id: &'a Uuid) -> ApiResult<Option<Routine>> {
        let routine = sqlx::query_as::<_, Routine>(
            r#"SELECT id as "id: uuid::Uuid", title, color, user_id as "user_id: uuid::Uuid", created_at, updated_at FROM routine WHERE id = ?"#
        ).bind(id)
        .fetch_optional(&self.db)
        .await?;
        Ok(routine)
    }

    async fn get_routines<'a>(&'a self, user_id: &'a Uuid) -> ApiResult<Vec<Routine>> {
        let routine = sqlx::query_as::<_, Routine>(
            r#"SELECT id as "id: uuid::Uuid", title, color, user_id as "user_id: uuid::Uuid", created_at, updated_at FROM routine WHERE user_id = ?"#
        ).bind(user_id)
        .fetch_all(&self.db)
        .await?;
        Ok(routine)
    }

    async fn create_routine<'a>(
        &'a self,
        title: &'a str,
        color: &'a str,
        user_id: &'a Uuid,
    ) -> ApiResult<Uuid> {
        let id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();
        sqlx::query(r#"INSERT INTO routine (id, title, color, user_id, created_at) VALUES ($1, $2, $3, $4, $5)"#)
            .bind(id)
            .bind(title)
            .bind(color)
            .bind(user_id)
            .bind(now)
            .execute(&self.db)
            .await?;
        Ok(id)
    }

    async fn delete_routine<'a>(&'a self, id: &'a Uuid) -> ApiResult<()> {
        sqlx::query(r#"DELETE FROM routine WHERE id = ?"#)
            .bind(id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}
