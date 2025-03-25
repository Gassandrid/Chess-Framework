mod api;
mod chess;

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use axum_server;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::handlers;
use crate::chess::game::Game;

// for initializing the server
#[tokio::main]
async fn main() {
    // boilerplate for setting up tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // shared game "state"
    let game = Arc::new(Mutex::new(Game::new()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // api routes
    let app = Router::new()
        .route("/api/chess/legal-moves", post(handlers::legal_moves))
        .route("/api/chess/make-move", post(handlers::make_move))
        .route("/api/chess/new-game", post(handlers::new_game))
        .route("/api/chess/undo-move", post(handlers::undo_move))
        .layer(Extension(game))
        .layer(cors);

    // run server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::info!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
