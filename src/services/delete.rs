use axum::{extract::Path, extract::State, http::Response};
use serde_json::json;

use crate::models::SharedState;

/// Delete a note by it's ID.
#[axum::debug_handler]
pub async fn delete(State(state): State<SharedState>, Path(id): Path<String>) -> Response<String> {
    let response = state
        .db
        .delete(format!("notes/{}", id).as_str())
        .await
        .unwrap();

    if response.status() != reqwest::StatusCode::OK {
        return Response::builder()
            .status(400)
            .header("Content-Type", "application/json")
            .body(json!({"error": "Failed to delete note"}).to_string())
            .unwrap();
    }

    Response::builder()
        .header("Content-Type", "application/json")
        .body(json!({ "message": format!("Note {id} deleted.") }).to_string())
        .unwrap()
}
