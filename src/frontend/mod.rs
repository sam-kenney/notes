use axum::response::Response;
pub mod index;
pub mod view_note;

pub async fn css_get() -> Response<String> {
    let markup = tokio::fs::read_to_string("static/index.css").await.unwrap();

    crate::build_response(markup, "text/css")
}
