use axum::{extract::Path, extract::State, http::Response};
use serde_json::json;

use crate::models::SharedState;

/// Get a note by it's ID.
/// If the ID is not found, return a 404.
#[axum::debug_handler]
pub async fn get(State(state): State<SharedState>, Path(id): Path<String>) -> Response<String> {
    let note = state.notes.iter().find(|note| note.id == id);

    if let Some(note) = note {
        return Response::builder()
            .header("Content-Type", "application/json")
            .body(json!(note).to_string())
            .unwrap();
    }

    crate::services::not_found().await
}
