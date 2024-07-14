use std::env;

use axum::async_trait;
use axum_login::{AuthnBackend, UserId};
use moka::future::Cache;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, url::Url, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use tracing::{instrument, Level};
use uuid::Uuid;

use super::user::User;
use crate::{api::OAuthClient, Client, Result};

#[derive(Clone)]
pub struct Backend {
    cache: Cache<Uuid, User>,
    oauth_client: BasicClient,
}

impl Backend {
    pub fn new() -> Self {
        let oauth_client = Self::media_wiki_client();

        Self {
            cache: Cache::new(1000),
            oauth_client,
        }
    }

    fn media_wiki_client() -> BasicClient {
        let client_id = env::var("CLIENT_ID")
            .map(ClientId::new)
            .expect("CLIENT_ID should be provided.");

        let client_secret = env::var("CLIENT_SECRET")
            .map(ClientSecret::new)
            .expect("CLIENT_SECRET should be provided");

        let auth_url =
            AuthUrl::new("https://www.mediawiki.org/w/rest.php/oauth2/authorize".to_string())
                .expect("This URL is valid.");

        let token_url =
            TokenUrl::new("https://www.mediawiki.org/w/rest.php/oauth2/access_token".to_string())
                .expect("This URL is valid.");

        BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
    }

    pub fn oauth_url(&self) -> (Url, CsrfToken) {
        self.oauth_client.authorize_url(CsrfToken::new_random).url()
    }
}

#[derive(Clone, Deserialize)]
pub enum Credentials {
    Developer {
        token: String,
    },
    Oauth {
        code: String,
        old_state: CsrfToken,
        new_state: CsrfToken,
    },
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

        let token = match creds {
            Credentials::Developer { token } => token,
            Credentials::Oauth {
                code,
                old_state,
                new_state,
            } => {
                // Ensure the CSRF state has not been tampered with.
                if old_state.secret() != new_state.secret() {
                    return Ok(None);
                };

                let response = self
                    .oauth_client
                    .exchange_code(AuthorizationCode::new(code))
                    .request_async(async_http_client)
                    .await?;

                String::from(response.access_token().secret())
            }
        };

        let client = Client::new();
        let oauth = OAuthClient::new(&client, &token);

        let Some(username) = oauth.username().await? else {
            return Ok(None);
        };

        let user = User::new(id, username, token);

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
