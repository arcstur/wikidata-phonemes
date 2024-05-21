mod app;
mod auth;
mod error;

use tracing::info;

use app::App;
pub use app::AppRouter;
pub use error::{Error, Result};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()) // allows using RUST_LOG env var
        .with_ansi(false) // colors don't work well when piping to grep, filtering, etc.
        .init();

    let app = App::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("web server started.");
    axum::serve(listener, app.into_router().await)
        .await
        .unwrap();
}
