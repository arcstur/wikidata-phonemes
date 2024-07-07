mod templates;

use axum::{
    extract::{Path, State},
    routing::get,
};

use crate::{phonemes::Phoneme, AppRouter, Client, EntityId, Result, WikiValue};
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

async fn single_language(State(client): State<Client>, Path(id): Path<String>) -> Result<Details> {
    let phonemes = Phoneme::by_language(&client, EntityId(id)).await?;
    Ok(Details { phonemes })
}

#[derive(Debug, Deserialize)]
struct Language {
    #[serde(rename = "language")]
    q: EntityId,
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
