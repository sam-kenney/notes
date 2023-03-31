use askama::Template;
use axum::{extract::Path, response::Response};

#[derive(Template)]
#[template(path = "view_note.html")]
pub struct IndexTemplate {
    note: crate::models::Note,
}

async fn get_note(id: String) -> crate::models::Note {
    reqwest::get(format!("http://localhost:8080/api/notes/{}/", id))
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn html_get(Path(id): Path<String>) -> Response<String> {
    let markup: String = IndexTemplate {
        note: get_note(id).await,
    }
    .render()
    .unwrap();

    crate::build_response(markup, "text/html")
}
