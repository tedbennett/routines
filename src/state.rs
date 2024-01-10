use async_session::MemoryStore;
use axum::extract::FromRef;
use clap::Parser;
use oauth2::basic::BasicClient;
use reqwest::Client;

use crate::database::{DataLayer, Database};

#[derive(Clone)]
pub struct AppState<T: DataLayer> {
    pub db: T,
    pub session_store: MemoryStore,
    pub oauth_client: BasicClient,
    pub http_client: Client,
    pub env: Env,
}

impl<T: DataLayer> AppState<T> {
    pub fn new(db: T, env: Env, oauth_client: BasicClient) -> Self {
        Self {
            db,
            env,
            oauth_client,
            session_store: MemoryStore::new(),
            http_client: Client::new(),
        }
    }
}

impl<T: DataLayer> FromRef<AppState<T>> for MemoryStore {
    fn from_ref(state: &AppState<T>) -> Self {
        state.session_store.clone()
    }
}

impl<T: DataLayer> FromRef<AppState<T>> for BasicClient {
    fn from_ref(state: &AppState<T>) -> Self {
        state.oauth_client.clone()
    }
}

impl FromRef<AppState<Database>> for Database {
    fn from_ref(state: &AppState<Database>) -> Self {
        state.db.clone()
    }
}

#[derive(Parser, Clone, Debug)]
pub struct Env {
    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env)]
    pub port: usize,

    #[clap(long, env)]
    pub client_id: String,

    #[clap(long, env)]
    pub client_secret: String,

    #[clap(long, env)]
    pub redirect_url: String,

    #[clap(long, env)]
    pub auth_url: String,

    #[clap(long, env)]
    pub token_url: String,
}
