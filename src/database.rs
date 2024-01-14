use std::fmt::Debug;

use sqlx::{Pool, Sqlite};

use crate::models::{
    entries::RoutineEntryDataLayer, routines::RoutineDataLayer, sessions::SessionDataLayer,
    users::UserDataLayer,
};

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

impl DataLayer<'_> for Database {}
