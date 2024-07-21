use axum::{async_trait, extract::FromRequestParts, http::request::Parts, response::Redirect};
use axum_login::AuthUser;

use super::AuthSession;

#[derive(Clone)]
pub struct User {
    id: i64,
    username: String,
    token: String,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.token.as_bytes()
    }
}

impl User {
    pub(super) fn new(id: i64, username: String, token: String) -> Self {
        Self {
            id,
            username,
            token,
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("token", &"[redacted]")
            .finish()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = Redirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = AuthSession::from_request_parts(parts, state)
            .await
            .expect("Authentication layer should be installed");

        if let Some(user) = session.user {
            return Ok(user);
        } else {
            return Err(Redirect::to("/auth/login"));
        }
    }
}
