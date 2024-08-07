mod backend;
mod handlers;
mod oauth;
mod sessions;
mod templates;
mod user;

use axum::routing::{get, post};
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use sqlx::SqlitePool;

pub use self::{backend::Backend, sessions::Sessions, user::User};
use crate::AppRouter;

pub type AuthSession = axum_login::AuthSession<Backend>;

pub async fn layer(pool: SqlitePool) -> AuthManagerLayer<Backend, Sessions> {
    let session_layer = Sessions::layer(pool.clone()).await;
    let backend = Backend::new(pool);

    AuthManagerLayerBuilder::new(backend, session_layer).build()
}

pub fn router() -> AppRouter {
    AppRouter::new()
        .route("/login", get(handlers::login_get))
        .route(
            "/login/dev",
            get(handlers::login_dev_get).post(handlers::login_dev),
        )
        .route("/logout", get(handlers::logout))
        .route("/profile", get(handlers::profile))
        .route("/callback", get(oauth::callback))
        .route("/redirect", post(oauth::redirect))
}
