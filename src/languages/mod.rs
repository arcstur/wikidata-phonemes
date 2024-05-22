mod templates;

use axum::{
    extract::{Path, State},
    routing::get,
};

use crate::{AppRouter, Client, Phoneme, Result, WikiValue, WikidataQ};
use serde::Deserialize;
use templates::List;

use self::templates::Details;

pub fn router() -> AppRouter {
    AppRouter::new()
        .route("/", get(list_languages))
        .route("/:id", get(single_language))
}

async fn list_languages(State(client): State<Client>) -> Result<List> {
    let languages = Language::list(&client).await?;
    Ok(List { languages })
}

async fn single_language(State(client): State<Client>, Path(id): Path<u32>) -> Result<Details> {
    let phonemes = Phoneme::by_language(&client, WikidataQ(id)).await?;
    Ok(Details { phonemes })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn list_languages() {
        let client = Client::new();
        Language::list(&client).await.unwrap();
    }
}
