use axum::Json;
use serde_json::{Value, json};

pub async fn health_check() -> &'static str {
    tracing::info!("Health check requested");

    "RustForge API is healthy!"
}

pub async fn readiness_check() -> &'static str {
    tracing::info!("Readiness check requested");

    "RustForge API is ready!"
}

pub async fn hello() -> Json<Value> {
    tracing::info!("Hello endpoint requested");

    Json(json!({
        "message": "Hello from RustForge API!",
        "version": "1.0"
    }))
}
