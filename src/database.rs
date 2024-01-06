use sqlx::{Pool, Sqlite};

use crate::models::{routines::RoutineDataLayer, users::UserDataLayer};

pub trait DataLayer: Clone + UserDataLayer + RoutineDataLayer {}

#[derive(Clone)]
pub struct Database {
    pub db: Pool<Sqlite>,
}

impl Database {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }
}

impl DataLayer for Database {}
