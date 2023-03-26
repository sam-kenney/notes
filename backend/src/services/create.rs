use axum::{extract::State, http::Response, Json};
use serde_json::json;

use crate::models::{Note, SharedState};

/// Create a new note.
#[axum::debug_handler]
pub async fn create(
    State(state): State<SharedState>,
    Json(mut payload): Json<Note>,
) -> Response<String> {
    if payload.id.is_none() {
        payload = Note::new(payload.title.as_str(), payload.body.as_str(), None, None);
    };
    if payload.timestamp.is_none() {
        payload.timestamp = Some(chrono::Utc::now().timestamp());
    };

    let body = json!(payload).to_string();

    let response = state
        .db
        .patch(
            format!("notes/{}", payload.id.clone().unwrap()).as_str(),
            body.as_str(),
        )
        .await
        .unwrap();

    if response.status() != reqwest::StatusCode::OK {
        return Response::builder()
            .status(400)
            .header("Content-Type", "application/json")
            .body(json!({"error": "Failed to create note"}).to_string())
            .unwrap();
    }

    Response::builder()
        .header("Content-Type", "application/json")
        .body(json!(payload).to_string())
        .unwrap()
}
