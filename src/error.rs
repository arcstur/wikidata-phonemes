use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};
use oauth2::{basic::BasicRequestTokenError, reqwest::AsyncHttpClientError};

pub type Result<T> = std::result::Result<T, Error>;
use Error::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to communicate with the Wikidata API: {0}")]
    Client(#[from] reqwest::Error),

    // auth
    #[error("OAuth callback was reached but the user does not have the original CSRF state in its session.")]
    MissingOldState,
    #[error("Authorization failed.")]
    AuthorizationFailed,

    // external
    #[error(transparent)]
    Session(#[from] tower_sessions::session::Error),

    #[error("There was an error communicating with the Mediawiki's OAuth API: {0}")]
    Oauth(#[from] BasicRequestTokenError<AsyncHttpClientError>),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!(error = ?self);

        let status_code = match self {
            MissingOldState | AuthorizationFailed => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match self {
            AuthorizationFailed => "Authorization failed.",
            _ => "An error happened. Please, try again.",
        };

        let template = ErrorTemplate { message };

        (status_code, template).into_response()
    }
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    message: &'static str,
}
