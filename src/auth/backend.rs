use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use moka::future::Cache;
use serde::Deserialize;
use tracing::{instrument, Level};
use uuid::Uuid;

use crate::{Client, Result};

#[derive(Clone)]
pub struct User {
    id: Uuid,
    username: String,
    token: String,
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.token.as_bytes()
    }
}

impl User {
    pub fn username(&self) -> &str {
        &self.username
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
                let username = client.username(&token).await?;
                User {
                    id,
                    username,
                    token,
                }
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
