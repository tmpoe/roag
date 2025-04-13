use axum::{extract::{Path}, routing::{get, post}, Router};


pub async fn create_room() -> String {
  println!("Creating room...");
  "Creating room...".to_string()
}


pub async fn get_room(Path(room_id): Path<String>) -> String {
    println!("Getting room with id: {}", room_id);
    format!("Getting room with id: {}", room_id)
}

pub fn room_router() -> Router {
    let version_prefix = "/v1";
    Router::new()
        .route(format!("{version_prefix}/room").as_str(), post(create_room))
        .route(format!("{version_prefix}/room/{{room_id}}").as_str(), get(get_room))
}