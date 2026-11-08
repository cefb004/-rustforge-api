use axum::{
    routing::get,
    Router,
};

use super::handlers::{
    health_check,
    readiness_check,
    hello,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        .route("/api/v1/hello", get(hello))
}
