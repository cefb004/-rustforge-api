use axum::{Router, http::Response, routing::get};

use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing::Span;

use super::handlers::{health_check, hello, readiness_check};

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        .route("/api/v1/hello", get(hello))
        .layer(TraceLayer::new_for_http().on_response(
            |response: &Response<_>, latency: Duration, _span: &Span| {
                let status = response.status();
                let latency_ms = latency.as_millis();

                if status.is_server_error() {
                    tracing::error!(
                        status = %status,
                        latency_ms,
                        "HTTP server error"
                    );
                } else if status.is_client_error() {
                    tracing::warn!(
                        status = %status,
                        latency_ms,
                        "HTTP client error"
                    );
                } else {
                    tracing::info!(
                        status = %status,
                        latency_ms,
                        "HTTP response"
                    );
                }
            },
        ))
}
