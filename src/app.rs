use axum::{extract::FromRef, routing::get, Router};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use tower_http::{services::ServeDir, trace::TraceLayer};
type Request = axum::http::Request<axum::body::Body>;

use super::User;
use crate::Client;

pub type AppRouter = Router<AppState>;

#[derive(Clone, FromRef)]
pub struct AppState {
    client: Client,
    pub pool: SqlitePool,
}

impl AppState {
    async fn new() -> Self {
        Self {
            client: Client::new(),
            pool: Self::pool().await,
        }
    }

    async fn pool() -> SqlitePool {
        let options = SqliteConnectOptions::new()
            .filename("phonemes.db")
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .expect("can't connect to the database");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("failed to run migrations");

        pool
    }
}

#[derive(Clone)]
pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn into_router(self) -> Router {
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(|_req: &_| tracing::debug_span!("request", id = %uuid::Uuid::new_v4()))
            .on_request(|req: &Request, _span: &_| {
                tracing::debug!(method = %req.method(), uri = %req.uri(), "started");
            });

        let state = AppState::new().await;
        let pool = state.pool.clone();

        let auth_layer = super::auth::layer(pool).await;

        Router::new()
            .route("/", get(index))
            .nest("/auth", super::auth::router())
            .nest("/languages", super::languages::router())
            .nest("/phonemes", super::phonemes::router())
            .nest("/activity", super::activity::router())
            .with_state(state)
            .nest_service("/static", ServeDir::new("static"))
            .layer(auth_layer)
            .layer(trace_layer)
    }
}

async fn index(user: Option<User>) -> Index {
    Index { user }
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct Index {
    user: Option<User>,
}
