use axum::{routing::get, Router};

pub async fn health_check() -> String {
    println!("HC called");
    "Pong".to_string()
}

pub fn health_check_router() -> Router {
    Router::new().route("/health_check", get(health_check))
}