use axum::{extract::State, http::Response};
use serde_json::json;

use crate::models::SharedState;

/// List all notes.
#[axum::debug_handler]
pub async fn list(State(state): State<SharedState>) -> Response<String> {
    Response::builder()
        .header("Content-Type", "application/json")
        .body(json!(state.notes).to_string())
        .unwrap()
}
