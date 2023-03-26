use axum::{extract::Path, extract::State, http::Response};
use serde_json::json;

use crate::models::{Note, SharedState};

/// Get a note by it's ID.
/// If the ID is not found, return a 404.
#[axum::debug_handler]
pub async fn get(State(state): State<SharedState>, Path(id): Path<String>) -> Response<String> {
    let response = state.db.get(&format!("notes/{}", id)).await.unwrap();
    if response.status() != reqwest::StatusCode::OK {
        return crate::services::not_found().await;
    }

    let note = match response.json::<Note>().await {
        Ok(note) => note,
        Err(_) => return crate::services::not_found().await,
    };

    Response::builder()
        .header("Content-Type", "application/json")
        .body(json!(note).to_string())
        .unwrap()
}
