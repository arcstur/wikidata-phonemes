use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use oauth2::CsrfToken;
use serde::Deserialize;
use tower_sessions::Session;

use super::{backend::Credentials, AuthSession};
use crate::{Error, Result};

const CSRF_OLD_STATE: &str = "csrf_old_state";

#[derive(Debug, Clone, Deserialize)]
pub struct Authorization {
    code: String,
    new_state: CsrfToken,
}

pub(super) async fn callback(
    mut auth_session: AuthSession,
    session: Session,
    Query(authorization): Query<Authorization>,
) -> Result<Response> {
    let old_state = session
        .get(CSRF_OLD_STATE)
        .await?
        .ok_or(Error::MissingOldState)?;

    let creds = Credentials::Oauth {
        code: authorization.code,
        old_state,
        new_state: authorization.new_state,
    };

    if let Some(user) = auth_session.authenticate(creds).await? {
        auth_session.login(&user).await?;
        Ok(Redirect::to("/auth/profile").into_response())
    } else {
        Ok((
            StatusCode::UNAUTHORIZED,
            "Authorization with Wikimedia account failed.",
        )
            .into_response())
    }
}

pub(super) async fn redirect(auth_session: AuthSession, session: Session) -> Result<Redirect> {
    let (auth_url, old_state) = auth_session.backend.oauth_url();

    session.insert(CSRF_OLD_STATE, old_state.secret()).await?;

    Ok(Redirect::to(auth_url.as_str()))
}
