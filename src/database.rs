use crate::models::{
    entries::RoutineEntryDataLayer, routines::RoutineDataLayer, sessions::SessionDataLayer,
    users::UserDataLayer,
};
use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::fmt::Debug;
use tokio::fs::OpenOptions;

pub trait DataLayer<'a>:
    Clone
    + Debug
    + std::marker::Send
    + std::marker::Sync
    + UserDataLayer
    + RoutineDataLayer
    + SessionDataLayer
    + RoutineEntryDataLayer
    + 'a
{
}

#[derive(Clone, Debug)]
pub struct Database {
    pub db: Pool<Sqlite>,
}

impl Database {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }
}

pub async fn setup_database(path: &str) -> Result<Pool<Sqlite>> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .await?;
    let database_url = format!("sqlite://{}", path);
    let pool = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

impl DataLayer<'_> for Database {}
