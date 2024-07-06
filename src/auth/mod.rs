mod backend;
mod sessions;
mod templates;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Form,
};
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use serde::Deserialize;

use crate::AppRouter;
use backend::Credentials;
use sessions::Sessions;
use templates::{Login, LoginDev, Profile};

pub use backend::{Backend, User};
pub type AuthSession = axum_login::AuthSession<Backend>;

pub async fn layer() -> AuthManagerLayer<Backend, Sessions> {
    let session_layer = Sessions::layer().await;
    let backend = Backend::new();

    AuthManagerLayerBuilder::new(backend, session_layer).build()
}

pub fn router() -> AppRouter {
    AppRouter::new()
        .route("/login", get(|| async { Login {} }).post(login))
        .route("/login/dev", get(|| async { LoginDev {} }).post(login_dev))
        .route("/logout", get(logout))
        .route("/profile", get(profile))
}

async fn login() -> impl IntoResponse {
    Redirect::to("/")
}

#[derive(Deserialize)]
struct Token {
    token: String,
}

async fn login_dev(mut session: AuthSession, Form(token): Form<Token>) -> Response {
    let token = token.token;
    let creds = Credentials::from_token(token);

    if let Some(user) = session.authenticate(creds).await.unwrap() {
        session.login(&user).await.unwrap();
        Redirect::to("/auth/profile").into_response()
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

async fn logout(mut session: AuthSession) -> impl IntoResponse {
    session.logout().await.unwrap();
    Redirect::to("/")
}

async fn profile(session: AuthSession) -> Response {
    if let Some(user) = &session.user {
        Profile { user }.into_response()
    } else {
        Redirect::to("/auth/login").into_response()
    }
}
