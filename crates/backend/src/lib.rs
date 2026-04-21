use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

pub fn create_api_router() -> Router {
    Router::new().route("/health", get(|| async { "OK" }))
}

pub fn create_app() -> Router {
    Router::new()
        .nest("/api", create_api_router())
        .route("/", get(|| async { "Hello CI/CD!" }))
        .fallback_service(ServeDir::new("dist"))
}

pub async fn run_server() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080)); // 改为8081端口
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind port");

    println!("Server running on http://{}", addr);

    axum::serve(listener, create_app()).await.unwrap();
}
