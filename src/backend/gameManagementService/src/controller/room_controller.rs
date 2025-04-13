use axum::{extract::{Path, Query}, routing::{get, post}, Router};
use chrono::{DateTime, Utc};

#[derive(serde::Deserialize, Default, Debug)]
#[serde(default)]
pub struct SearchRoomsQuery {
  creator: Option<String>,
  num_players: Option<i32>,
  created_at: Option<DateTime<Utc>>,
  closed_at: Option<DateTime<Utc>>,
}


pub async fn create_room() -> String {
  println!("Creating room...");
  "Creating room...".to_string()
}


pub async fn get_room(Path(room_id): Path<String>) -> String {
    println!("Getting room with id: {}", room_id);
    format!("Getting room with id: {}", room_id)
}

pub async fn search_rooms(query_params: Query<SearchRoomsQuery>) -> String {
  format!(
    "Query params: creator={:?}, num_players={:?}, created_at={:?}, closed_at={:?}",
    query_params.creator,
    query_params.num_players,
    query_params.0.created_at,
    query_params.0.closed_at,
)
}

pub fn room_router() -> Router {
    let version_prefix = "/v1";
    Router::new()
        .route(format!("{version_prefix}/room").as_str(), post(create_room))
        .route(format!("{version_prefix}/room").as_str(), get(search_rooms))
        .route(format!("{version_prefix}/room/{{room_id}}").as_str(), get(get_room))
}