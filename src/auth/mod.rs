mod backend;
mod sessions;
mod templates;
mod user;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Form,
};
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use serde::Deserialize;

use crate::{AppRouter, Result};
use backend::Credentials;
use sessions::Sessions;
use templates::{Login, LoginDev, Profile};

pub use self::{backend::Backend, user::User};
pub type AuthSession = axum_login::AuthSession<Backend>;

pub async fn layer() -> AuthManagerLayer<Backend, Sessions> {
    let session_layer = Sessions::layer().await;
    let backend = Backend::new();

    AuthManagerLayerBuilder::new(backend, session_layer).build()
}

pub fn router() -> AppRouter {
    AppRouter::new()
        .route("/login", get(login_get))
        .route("/login/dev", get(|| async { LoginDev {} }).post(login_dev))
        .route("/logout", get(logout))
        .route("/profile", get(profile))
}

async fn login_get(session: AuthSession) -> Response {
    if session.user.is_some() {
        Redirect::to("/auth/profile").into_response()
    } else {
        Login {}.into_response()
    }
}

#[derive(Deserialize)]
struct Token {
    token: String,
}

async fn login_dev(mut session: AuthSession, Form(token): Form<Token>) -> Result<Response> {
    let token = token.token;
    let creds = Credentials::from_token(token);

    if let Some(user) = session.authenticate(creds).await? {
        session.login(&user).await?;
        Ok(Redirect::to("/auth/profile").into_response())
    } else {
        Ok((StatusCode::UNAUTHORIZED, "Developer authorization failed.").into_response())
    }
}

async fn logout(mut session: AuthSession) -> impl IntoResponse {
    session.logout().await.unwrap();
    Redirect::to("/")
}

async fn profile(user: User) -> Response {
    Profile { user: &user }.into_response()
}
