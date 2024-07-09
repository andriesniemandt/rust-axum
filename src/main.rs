use axum;
use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let srv = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Unable to connect to the server");

    let routes = Router::new()
        .route("/", get(|| async { "Hello from Rust-Axum" }));

    axum::serve(srv, routes)
        .await
        .expect("Error serving application");
}