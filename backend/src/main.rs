mod firebase;
mod models;
mod services;
use axum::{
    routing::{delete, get, post, put},
    Router, Server,
};
use std::sync::Arc;

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

    // TODO: Add routes & methods for POST, PATCH, DELETE
    let router = Router::new()
        .route("/api/notes/", get(services::list))
        .route("/api/notes/", post(services::create))
        .route("/api/notes/:id/", get(services::get))
        .route("/api/notes/:id/", put(services::update))
        .route("/api/notes/:id/", delete(services::delete))
        .with_state(app_state.clone())
        .fallback(services::not_found);

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());

    let addr = server.local_addr();

    println!("Listening on {addr}");

    match server.await {
        Ok(_) => println!("Server stopped gracefully"),
        Err(err) => eprintln!("Server stopped with error: {err}"),
    }
}
