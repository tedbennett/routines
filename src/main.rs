use std::env;

use axum::{routing::get, Router};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    let _ = dotenv();
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));
    let port = env::var("PORT").unwrap_or("8000".to_string());
    let address = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
