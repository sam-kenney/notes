use axum::{
    extract::{Path, State},
    http::Response,
    Json,
};
use serde_json::json;

use crate::models::{Note, SharedState};

/// Update a note by it's ID.
#[axum::debug_handler]
pub async fn update(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(mut payload): Json<Note>,
) -> Response<String> {
    if payload.id.is_none() {
        payload.id = Some(id.clone());
    };
    if payload.timestamp.is_none() {
        payload.timestamp = Some(chrono::Utc::now().timestamp());
    };

    let body = json!(payload).to_string();

    let response = state
        .db
        .patch(format!("notes/{}", id).as_str(), body.as_str())
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
