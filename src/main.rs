mod model_manager;
mod handlers;

use axum::{
    routing::{get, post_service, post},
    Router,
};
use std::net::SocketAddr;
use dotenvy::dotenv;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // 1. Load environment variables
    dotenv().ok();

    // 2. Build our application with routes
    let app = Router::new()
        .route("/", get(home))
        .route("/assistant/info", get(handlers::info_page_handler))
        .route("/assistant/chat", get(handlers::chat_page_handler))
        .route("/assistant/api/chat", post(handlers::chat_api_handler))
        .nest_service("/assets", ServeDir::new("assets"));

    // 3. Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Hello from Rust Backend 🚀\nGo to /assistant/info or /assistant/chat"
}
