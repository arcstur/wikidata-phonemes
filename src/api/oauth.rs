use serde::Deserialize;
use tracing::instrument;

use super::Client;

pub struct OAuthClient<'a> {
    client: &'a Client,
    token: &'a str,
}

impl<'a> OAuthClient<'a> {
    const MEDIAWIKI: &'static str = "https://www.mediawiki.org/w/rest.php";
    const PROFILE: &'static str = "/oauth2/resource/profile";

    pub fn new(client: &'a Client, token: &'a str) -> Self {
        Self { client, token }
    }

    /// Tries to obtain an username from the provided access token.
    /// Returns `None` when unauthorized.
    #[instrument(level = "debug", skip(self), err)]
    pub async fn username(&self) -> reqwest::Result<Option<String>> {
        Ok(self
            .client
            .inner
            .get(format!("{}{}", Self::MEDIAWIKI, Self::PROFILE))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<UsernameResponse>()
            .await
            .ok()
            .map(|res| res.username))
    }
}

#[derive(Debug, Deserialize)]
struct UsernameResponse {
    username: String,
}
