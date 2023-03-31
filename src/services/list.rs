use std::collections::HashMap;

use axum::{extract::State, http::Response};
use serde_json::json;

use crate::models::{Note, SharedState};

/// List all notes.
#[axum::debug_handler]
pub async fn list(State(state): State<SharedState>) -> Response<String> {
    let response = state.db.get("notes").await.unwrap();
    let body = response.json::<HashMap<String, Note>>().await.unwrap();

    let notes = body
        .into_iter()
        .map(|(id, note)| {
            let mut note = note;
            note.id = id;
            note
        })
        .collect::<Vec<Note>>();

    Response::builder()
        .header("Content-Type", "application/json")
        .body(json!(notes).to_string())
        .unwrap()
}
