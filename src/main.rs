use rustforge_api::api::routes::create_router;
use tokio::net::TcpListener;

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize structured logging
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("rustforge_api=info,tower_http=debug"));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = create_router();

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TCP listener");

    tracing::info!("RustForge API running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
