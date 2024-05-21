mod templates;

use axum::{extract::State, routing::get};

use crate::{AppRouter, Client, Result, WikiValue};
use serde::Deserialize;
use templates::List;

pub fn router() -> AppRouter {
    AppRouter::new().route("/", get(list_languages))
}

#[derive(Debug, Deserialize)]
struct Language {
    #[serde(rename = "language")]
    uri: WikiValue<String>,
    #[serde(rename = "languageLabel")]
    label: WikiValue<String>,
    phoneme_count: WikiValue<String>,
}

impl Language {
    const LIST: &'static str = include_str!("languages_phoneme_count.sparql");

    async fn list(client: &Client) -> Result<Vec<Self>> {
        let query = Self::LIST;
        Ok(client.query::<Self>(query).await?)
    }
}

async fn list_languages(State(client): State<Client>) -> Result<List> {
    let languages = Language::list(&client).await?;
    Ok(List { languages })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn list_languages() {
        let client = Client::new();
        Language::list(&client).await.unwrap();
    }
}
