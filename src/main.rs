#![allow(async_fn_in_trait)]

use anyhow::Context;
use auth::{google_auth, login_authorized, logout, protected};
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use database::{setup_database, Database};
use dotenvy::dotenv;
use r#static::static_router;
use routes::{create_routine, root, toggle_entry};
use state::{AppState, Env};
use std::env;
use tower_http::trace::TraceLayer;

mod auth;
mod database;
mod error;
mod models;
mod routes;
mod state;
mod r#static;
mod templates;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv();
    let env = Env::parse();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let pool = setup_database(&env.database_path)
        .await
        .context("Failed to setup database")?;
    let oauth = auth::oauth_client(&env).expect("Failed to build oauth client");

    let state = AppState::new(Database::new(pool), env, oauth);
    let app = Router::new()
        .route("/", get(root))
        .route("/routine", post(create_routine))
        .route("/entry", post(toggle_entry))
        .route("/static/*path", get(static_router))
        .route("/auth/google", get(google_auth))
        .route("/auth/authorized", get(login_authorized))
        .route("/protected", get(protected))
        .route("/logout", get(logout))
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    let port = env::var("PORT").unwrap_or("8000".to_string());
    let address = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
