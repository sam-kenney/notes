use axum::http::Response;
use serde_json::json;

/// This is the default handler for all routes that are not defined.
/// It returns a 404 response with a JSON body.
#[axum::debug_handler]
pub async fn not_found() -> Response<String> {
    Response::builder()
        .header("Content-Type", "application/json")
        .status(404)
        .body(json!({"message": "Not Found"}).to_string())
        .unwrap()
}
