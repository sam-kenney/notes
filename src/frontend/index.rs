use askama::Template;
use axum::response::Response;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    notes: Vec<crate::models::Note>,
}

async fn get_notes() -> Vec<crate::models::Note> {
    reqwest::get("http://localhost:8080/api/notes/")
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn html_get() -> Response<String> {
    let markup: String = IndexTemplate {
        notes: get_notes().await,
    }
    .render()
    .unwrap();

    crate::build_response(markup, "text/html")
}
