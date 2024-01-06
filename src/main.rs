use std::env;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::{Pool, Sqlite, SqlitePool};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct AppState {
    db: Pool<Sqlite>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let app = Router::new()
        .route("/", get(root))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState { db: pool });

    let port = env::var("PORT").unwrap_or("8000".to_string());
    let address = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
