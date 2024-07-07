use axum::async_trait;
use axum_login::{AuthnBackend, UserId};
use moka::future::Cache;
use serde::Deserialize;
use tracing::{instrument, Level};
use uuid::Uuid;

use super::user::User;
use crate::{api::OAuthClient, Client, Result};

#[derive(Clone)]
pub struct Backend {
    cache: Cache<Uuid, User>,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(1000),
        }
    }
}

#[derive(Clone, Deserialize)]
pub enum Credentials {
    Developer { token: String },
}

impl Credentials {
    pub(super) fn from_token(token: String) -> Self {
        Self::Developer { token }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = crate::Error;

    async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>> {
        let id = Uuid::now_v7();
        let client = Client::new();
        let user = match creds {
            Self::Credentials::Developer { token } => {
                let oauth = OAuthClient::new(&client, &token);
                let Some(username) = oauth.username().await? else {
                    return Ok(None);
                };
                User::new(id, username, token)
            }
        };
        self.cache.insert(id, user.clone()).await;
        Ok(Some(user))
    }

    #[allow(clippy::blocks_in_conditions)]
    #[instrument(skip(self), fields(cache_hit), ret, err, level = Level::DEBUG)]
    async fn get_user(&self, id: &UserId<Self>) -> Result<Option<Self::User>> {
        Ok(self.cache.get(id).await)
    }
}

impl From<axum_login::Error<Backend>> for crate::Error {
    fn from(value: axum_login::Error<Backend>) -> Self {
        match value {
            axum_login::Error::Session(e) => Self::Session(e),
            axum_login::Error::Backend(e) => e,
        }
    }
}
