use sqlx::{Pool, Sqlite};

pub trait DataLayer: Clone {}

#[derive(Clone)]
pub struct Database {
    db: Pool<Sqlite>,
}

impl Database {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }
}

impl DataLayer for Database {}
