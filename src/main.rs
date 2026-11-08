use rustforge_api::api::routes::create_router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = create_router();

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TCP listener");

    println!("RustForge API running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
