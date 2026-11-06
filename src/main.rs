use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TCP listener");

    println!("RustForge API running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn health_check() -> &'static str {
    "RustForge API is healthy!"
}
