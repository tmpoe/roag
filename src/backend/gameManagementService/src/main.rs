
use axum::{Router, serve};
use tokio::net::TcpListener;
pub mod controller;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/api", controller::health_check_router());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    serve(listener, app).await.unwrap();
}