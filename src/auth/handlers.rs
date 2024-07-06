use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use serde::Deserialize;

use super::{
    backend::Credentials,
    templates::{Login, LoginDev, Profile},
    AuthSession, User,
};
use crate::Result;

pub(super) async fn login_get(session: AuthSession) -> Response {
    if session.user.is_some() {
        Redirect::to("/auth/profile").into_response()
    } else {
        Login {}.into_response()
    }
}

pub(super) async fn login_dev_get() -> LoginDev {
    LoginDev {}
}

#[derive(Deserialize)]
pub(super) struct Dev {
    token: String,
}

pub(super) async fn login_dev(mut session: AuthSession, Form(dev): Form<Dev>) -> Result<Response> {
    let token = dev.token;
    let creds = Credentials::from_token(token);

    if let Some(user) = session.authenticate(creds).await? {
        session.login(&user).await?;
        Ok(Redirect::to("/auth/profile").into_response())
    } else {
        Ok((StatusCode::UNAUTHORIZED, "Developer authorization failed.").into_response())
    }
}

pub(super) async fn logout(mut session: AuthSession) -> impl IntoResponse {
    session.logout().await.unwrap();
    Redirect::to("/")
}

pub(super) async fn profile(user: User) -> Response {
    Profile { user: &user }.into_response()
}
