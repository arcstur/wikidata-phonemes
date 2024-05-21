use axum::Router;
use tower_http::{services::ServeDir, trace::TraceLayer};
type Request = axum::http::Request<axum::body::Body>;

pub type AppRouter = Router<()>;

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

        let auth_layer = super::auth::layer().await;

        Router::new()
            // .route("/", get(index))
            .nest("/auth", super::auth::router())
            .nest_service("/static", ServeDir::new("static"))
            .layer(auth_layer)
            .layer(trace_layer)
    }
}
