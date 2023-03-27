mod firebase;
mod models;
mod services;
mod shutdown;
use axum::{
    routing::{delete, get, post, put},
    Router, Server,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::{filter, prelude::*};

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
    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", Level::TRACE)
        .with_target("tower_http::trace::on_request", Level::TRACE)
        .with_target("tower_http::trace::make_span", Level::DEBUG)
        .with_default(Level::INFO);
    let tracing_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .init();

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
        .fallback(services::not_found)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("Listening on {}", addr);

    let server = Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown::signal());

    match server.await {
        Ok(_) => println!("Server stopped gracefully"),
        Err(err) => eprintln!("Server error: {}", err),
    }
}
