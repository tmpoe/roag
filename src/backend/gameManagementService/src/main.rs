
use axum::{Router, serve};
use tokio::net::TcpListener;
pub mod controller;

#[tokio::main]
async fn main() {
    let api_prefix = "/api";
    let app = Router::new()
        .nest(api_prefix, controller::health_check_router())
        .nest(api_prefix, controller::room_router());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    serve(listener, app).await.unwrap();
}