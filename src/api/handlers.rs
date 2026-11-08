use axum::Json;
use serde_json::{json, Value};

pub async fn health_check() -> &'static str {
    "RustForge API is healthy!"
}

pub async fn readiness_check() -> &'static str {
    "RustForge API is ready!"
}

pub async fn hello() -> Json<Value> {
    Json(json!({
        "message": "Hello from RustForge API!",
        "version": "1.0"
    }))
}
