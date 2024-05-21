use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use moka::future::Cache;
use serde::Deserialize;
use tracing::{instrument, Level};
use uuid::Uuid;

use crate::Result;

#[derive(Clone)]
pub struct User {
    id: Uuid,
    username: String,
    password_hash: String,
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password_hash.as_bytes()
    }
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[redacted]")
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
            cache: Cache::new(500),
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = crate::Error;

    async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>> {
        let user = todo!("Implement OAuth");
        let id = Uuid::now_v7();
        self.cache.insert(id, user).await;
        Ok(Some(user))
    }

    #[instrument(skip(self), fields(cache_hit), ret, err, level = Level::DEBUG)]
    async fn get_user(&self, id: &UserId<Self>) -> Result<Option<Self::User>> {
        Ok(self.cache.get(id).await)
    }
}
