#![allow(async_fn_in_trait)]

use axum::{
    routing::{get, post},
    Router,
};
use database::{DataLayer, Database};
use dotenvy::dotenv;
use r#static::static_router;
use routes::{create_routine, root, toggle_entry};
use sqlx::SqlitePool;
use std::env;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

mod database;
mod error;
mod models;
mod routes;
mod r#static;
mod templates;

#[derive(Clone)]
pub struct AppState<T: DataLayer> {
    db: T,
    user_id: Uuid,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!().run(&pool).await?;
    let state = AppState {
        db: Database::new(pool),
        user_id: Uuid::parse_str(&env::var("USER_ID").unwrap()).unwrap(),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/routine", post(create_routine))
        .route("/entry", post(toggle_entry))
        .route("/static/*path", get(static_router))
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    let port = env::var("PORT").unwrap_or("8000".to_string());
    let address = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
