mod activity;
mod api;
mod app;
mod auth;
mod error;
mod languages;
mod phonemes;

use tracing::info;

pub use api::{Client, EntityId, WikiValue};
pub use app::AppRouter;
pub use auth::User;
pub use error::{Error, Result};
pub use phonemes::Phoneme;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()) // allows using RUST_LOG env var
        .with_ansi(false) // colors don't work well when piping to grep, filtering, etc.
        .init();

    let app = app::App::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    info!("web server started.");
    axum::serve(listener, app.into_router().await)
        .await
        .unwrap();
}
