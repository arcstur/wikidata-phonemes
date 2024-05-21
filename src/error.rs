use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to communicate with the Wikidata API: {0}")]
    Client(#[from] reqwest::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!(error = ?self);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorTemplate {
                message: "An unexpected error happened. Please, try again.",
            },
        )
            .into_response()
    }
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    message: &'static str,
}
