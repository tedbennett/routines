use sqlx::{prelude::FromRow, sqlite::SqliteRow, Row};
use time::Date;
use uuid::Uuid;

use crate::{database::Database, error::ApiResult};

#[derive(FromRow)]
pub struct RoutineEntry {
    pub date: Date,
    pub routine_id: Uuid,
}

pub trait RoutineEntryDataLayer {
    async fn get_entries<'a>(&'a self, routine_id: &'a Vec<Uuid>) -> ApiResult<Vec<RoutineEntry>>;
    async fn toggle_entries<'a>(&'a self, date: &'a Date, routine_id: &'a Uuid) -> ApiResult<bool>;
    async fn create_entry<'a>(&'a self, date: &'a Date, routine_id: &'a Uuid) -> ApiResult<()>;
    async fn delete_entry<'a>(&'a self, date: &'a Date, routine_id: &'a Uuid) -> ApiResult<()>;
}

impl Database {
    async fn entry_exists(&self, date: &Date, routine_id: &Uuid) -> ApiResult<bool> {
        let record: SqliteRow = sqlx::query(
            r#"SELECT COUNT(date) as count FROM routine_entry WHERE date = ? AND routine_id = ?"#,
        )
        .bind(date)
        .bind(routine_id)
        .fetch_one(&self.db)
        .await?;
        let count: i64 = record.try_get("count")?;
        Ok(count > 0)
    }
}

impl RoutineEntryDataLayer for Database {
    async fn get_entries<'a>(&'a self, routine_ids: &'a Vec<Uuid>) -> ApiResult<Vec<RoutineEntry>> {
        if routine_ids.is_empty() {
            return Ok(vec![]);
        }
        let params = format!("?{}", ", ?".repeat(routine_ids.len() - 1));
        let sql = format!(
            r#"
            SELECT 
                routine_id, 
                date 
            FROM 
                routine_entry 
            WHERE 
                routine_id IN ({})
            "#,
            params
        );
        let mut query = sqlx::query_as::<_, RoutineEntry>(&sql);
        for id in routine_ids {
            query = query.bind(id);
        }
        let routines = query.fetch_all(&self.db).await?;
        Ok(routines)
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
        sqlx::query(r#"delete from routine_entry where date = ? and routine_id = ?"#)
            .bind(date)
            .bind(routine_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    async fn toggle_entries<'a>(&'a self, date: &'a Date, routine_id: &'a Uuid) -> ApiResult<bool> {
        let exists = self.entry_exists(date, routine_id).await?;

        tracing::info!("{exists}");
        if exists {
            self.delete_entry(date, routine_id).await?;
            Ok(true)
        } else {
            self.create_entry(date, routine_id).await?;
            Ok(false)
        }
    }
}
