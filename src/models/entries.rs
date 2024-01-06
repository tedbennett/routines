use sqlx::prelude::FromRow;
use time::Date;
use uuid::Uuid;

use crate::{database::Database, error::ApiResult};

#[derive(FromRow)]
pub struct RoutineEntry {
    pub date: Date,
    pub routine_id: Uuid,
}

pub trait RoutineEntryDataLayer {
    async fn get_entries<'a>(&'a self, routine_id: &'a Uuid) -> ApiResult<Vec<RoutineEntry>>;
    async fn create_entry<'a>(&'a self, date: &'a Date, routine_id: &'a Uuid) -> ApiResult<()>;
    async fn delete_entry<'a>(&'a self, date: &'a Date, routine_id: &'a Uuid) -> ApiResult<()>;
}

impl RoutineEntryDataLayer for Database {
    async fn get_entries<'a>(&'a self, routine_id: &'a Uuid) -> ApiResult<Vec<RoutineEntry>> {
        let routine = sqlx::query_as::<_, RoutineEntry>(
            r#"SELECT routine_id as "routine_id: uuid::Uuid", date FROM routine_entry WHERE routine_id = ?"#
        )
        .bind(routine_id)
        .fetch_all(&self.db)
        .await?;
        Ok(routine)
    }

    async fn create_entry<'a>(&'a self, date: &'a Date, routine_id: &'a Uuid) -> ApiResult<()> {
        sqlx::query(r#"INSERT INTO routine_entry (date, routine_id) VALUES ($1, $2)"#)
            .bind(date)
            .bind(routine_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    async fn delete_entry<'a>(&'a self, date: &'a Date, routine_id: &'a Uuid) -> ApiResult<()> {
        sqlx::query(r#"DELETE FROM routine_entry WHERE date = ? AND routine_id = ?"#)
            .bind(date)
            .bind(routine_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}
