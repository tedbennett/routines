use axum::extract::FromRef;
use clap::Parser;
use oauth2::basic::BasicClient;
use reqwest::Client;

use crate::{
    auth::DBSessionStore,
    database::{DataLayer, Database},
};

#[derive(Clone)]
pub struct AppState<T: for<'a> DataLayer<'a>> {
    pub db: T,
    pub session_store: DBSessionStore<T>,
    pub oauth_client: BasicClient,
    pub http_client: Client,
    pub env: Env,
}

impl AppState<Database> {
    pub fn new(db: Database, env: Env, oauth_client: BasicClient) -> Self {
        Self {
            db: db.clone(),
            env,
            oauth_client,
            session_store: DBSessionStore { db },
            http_client: Client::new(),
        }
    }
}

impl<T: for<'a> DataLayer<'a>> FromRef<AppState<T>> for DBSessionStore<T> {
    fn from_ref(state: &AppState<T>) -> Self {
        state.session_store.clone()
    }
}

impl<T: for<'a> DataLayer<'a>> FromRef<AppState<T>> for BasicClient {
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
    pub database_path: String,

    #[clap(long, env)]
    pub port: usize,

    #[clap(long, env)]
    pub client_id: String,

    #[clap(long, env)]
    pub client_secret: String,

    #[clap(long, env)]
    pub client_url: String,

    #[clap(long, env)]
    pub redirect_url: String,

    #[clap(long, env)]
    pub auth_url: String,

    #[clap(long, env)]
    pub token_url: String,
}
