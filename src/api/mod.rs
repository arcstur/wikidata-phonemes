mod editing;
mod models;
mod oauth;
mod querying;

pub use editing::{AddPhoneme, EditingClient};
pub use models::{EntityId, WikiValue};
pub use oauth::OAuthClient;

/// HTTP client to communicate with the Wikidata API.
#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    const USER_AGENT: &'static str =
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    /// Constructs a new client with a default User-Agent.
    pub fn new() -> Self {
        let inner = reqwest::ClientBuilder::new()
            .user_agent(Self::USER_AGENT)
            .build()
            .expect("Failed to create reqwest Client");

        Self { inner }
    }
}
