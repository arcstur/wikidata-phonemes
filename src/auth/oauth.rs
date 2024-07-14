use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
};
use oauth2::CsrfToken;
use serde::Deserialize;
use tower_sessions::Session;

use super::{backend::Credentials, AuthSession};
use crate::{Error, Result};

const CSRF_ORIGINAL_STATE: &str = "csrf_original_state";

#[derive(Debug, Clone, Deserialize)]
pub struct Authorization {
    code: String,
    state: CsrfToken,
}

pub(super) async fn callback(
    mut auth_session: AuthSession,
    session: Session,
    Query(authorization): Query<Authorization>,
) -> Result<Response> {
    let original_state = session
        .get(CSRF_ORIGINAL_STATE)
        .await?
        .ok_or(Error::MissingOriginalState)?;

    let creds = Credentials::Oauth {
        code: authorization.code,
        original_state,
        incoming_state: authorization.state,
    };

    let user = auth_session
        .authenticate(creds)
        .await?
        .ok_or(Error::AuthorizationFailed)?;

    auth_session.login(&user).await?;
    Ok(Redirect::to("/auth/profile").into_response())
}

pub(super) async fn redirect(auth_session: AuthSession, session: Session) -> Result<Redirect> {
    let (auth_url, original_state) = auth_session.backend.oauth_url();

    session
        .insert(CSRF_ORIGINAL_STATE, original_state.secret())
        .await?;

    Ok(Redirect::to(auth_url.as_str()))
}
