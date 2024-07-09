use axum::{routing::get, Router};
use tokio::net::TcpListener;

mod health;

pub async fn serve(url: &str) {
    let listener = TcpListener::bind(url).await.unwrap();
    let service = Router::new().route("/health", get(health::handler));

    axum::serve(listener, service).await.expect("Server failed");
}
