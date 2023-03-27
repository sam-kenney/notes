mod firebase;
mod models;
mod services;
use axum::{
    routing::{delete, get, post, put},
    Router, Server,
};
use std::{net::SocketAddr, sync::Arc};

/// Shared application state.
///
/// # Parameters
///
/// * `firebase` - A Firebase client to interact with the database.
pub struct AppState {
    db: firebase::Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_state = Arc::new(AppState {
        db: firebase::Client::new().await,
    });

    let app = Router::new()
        .route("/api/notes/", get(services::list))
        .route("/api/notes/", post(services::create))
        .route("/api/notes/:id/", get(services::get))
        .route("/api/notes/:id/", put(services::update))
        .route("/api/notes/:id/", delete(services::delete))
        .with_state(app_state.clone())
        .fallback(services::not_found);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
