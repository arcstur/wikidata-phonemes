use serde::{de::DeserializeOwned, Deserialize};
use tracing::{debug, instrument};

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
    const ENDPOINT: &'static str = "https://query.wikidata.org/sparql";
    const MEDIAWIKI: &'static str = "https://www.mediawiki.org/w/rest.php/";
    const ENDPOINT_PROFILE: &'static str = "oauth2/resource/profile";

    /// Constructs a new client with a default User-Agent.
    pub fn new() -> Self {
        let inner = reqwest::ClientBuilder::new()
            .user_agent(Self::USER_AGENT)
            .build()
            .expect("Failed to create reqwest Client");

        Self { inner }
    }

    /// Sends a query to the Wikidata API and deserializes
    /// the response into a list of items of type `T`.
    #[instrument(level = "debug", skip(self), err)]
    pub async fn query<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        query: &str,
    ) -> reqwest::Result<Vec<T>> {
        debug!("sending request");
        let response = self
            .inner
            .get(Self::ENDPOINT)
            .query(&[("query", query), ("format", "json")])
            .send()
            .await?
            .json::<QueryResponse<T>>()
            .await?;

        Ok(response.results.bindings)
    }

    #[instrument(level = "debug", skip(self, token), err)]
    pub async fn username(&self, token: &str) -> reqwest::Result<String> {
        let response = self
            .inner
            .get(format!("{}{}", Self::MEDIAWIKI, Self::ENDPOINT_PROFILE))
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<UsernameResponse>()
            .await?;

        Ok(response.username)
    }
}

#[derive(Debug, Deserialize)]
struct UsernameResponse {
    username: String,
}

/// Response returned by the Wikidata API. Contains the results in `results`.
#[derive(Debug, Deserialize)]
struct QueryResponse<T> {
    results: QueryResults<T>,
}

/// List of items of type `T` resulting from a Wikidata Query.
#[derive(Debug, Deserialize)]
struct QueryResults<T> {
    bindings: Vec<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[tokio::test]
    async fn simple_query() {
        Client::new()
            .query::<Value>(r#"SELECT ?item WHERE { wd:Q1860 wdt:P2587 ?item. } LIMIT 1"#)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn malformed_query() {
        Client::new()
            .query::<Value>("malformed query")
            .await
            .unwrap_err();
    }
}
