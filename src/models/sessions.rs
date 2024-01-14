use anyhow::Result;
use async_trait::async_trait;
use sqlx::FromRow;

use crate::database::Database;

#[async_trait]
pub trait SessionDataLayer {
    async fn get_session(&self, id: &str) -> Result<Option<String>>;
    async fn insert_session(&self, id: &str, session: &str, expiry: Option<String>) -> Result<()>;
    async fn delete_session(&self, id: &str) -> Result<()>;
    async fn delete_all_sessions(&self) -> Result<()>;
}

#[async_trait]
impl SessionDataLayer for Database {
    async fn get_session(&self, id: &str) -> Result<Option<String>> {
        #[derive(FromRow)]
        pub struct SessionRow {
            pub session: String,
        }
        let session =
            sqlx::query_as::<_, SessionRow>(r#"SELECT session FROM session WHERE id = ?"#)
                .bind(id)
                .fetch_optional(&self.db)
                .await?;
        Ok(session.map(|s| s.session))
    }

    async fn insert_session(&self, id: &str, session: &str, expiry: Option<String>) -> Result<()> {
        sqlx::query(r#"INSERT INTO session (id, session, expiry) VALUES ($1, $2, $3)"#)
            .bind(id)
            .bind(session)
            .bind(expiry)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    async fn delete_session(&self, id: &str) -> Result<()> {
        sqlx::query(r#"DELETE FROM session WHERE id = ?"#)
            .bind(id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    async fn delete_all_sessions(&self) -> Result<()> {
        sqlx::query(r#"DELETE FROM session "#)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}
