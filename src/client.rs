use std::fmt::Display;

use serde::{de::DeserializeOwned, Deserialize};
use tracing::{debug, instrument};

pub struct WikidataQ(pub u32);

impl WikidataQ {
    pub fn as_str(&self) -> String {
        format!("Q{}", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct WikiValue<T> {
    #[serde(rename = "type")]
    pub kind: String,
    pub value: T,
}

impl<T: Display> Display for WikiValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// HTTP client to communicate with the Wikidata API.
#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    const USER_AGENT: &'static str =
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    const ENDPOINT: &'static str = "https://query.wikidata.org/sparql";

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
    #[instrument(level = "debug", skip(self), ret, err)]
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
