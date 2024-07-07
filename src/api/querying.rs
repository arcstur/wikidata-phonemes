use serde::{de::DeserializeOwned, Deserialize};
use tracing::{debug, instrument};

use super::Client;

impl Client {
    const QUERY_ENDPOINT: &'static str = "https://query.wikidata.org/sparql";

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
            .get(Self::QUERY_ENDPOINT)
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
