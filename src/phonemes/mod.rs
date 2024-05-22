mod templates;

use axum::{extract::State, routing::get};

use crate::{AppRouter, Client, Result, WikiValue, WikidataQ};
use serde::Deserialize;
use templates::List;

pub fn router() -> AppRouter {
    AppRouter::new().route("/", get(list_phonemes))
}

#[derive(Debug, Deserialize)]
pub struct Phoneme {
    #[serde(rename = "phoneme")]
    pub q: WikidataQ,
    #[serde(rename = "phonemeLabel")]
    pub label: WikiValue<String>,
    pub transcriptions: WikiValue<String>,
    pub audio: Option<WikiValue<String>>,
}

impl Phoneme {
    const LIST: &'static str = include_str!("list.sparql");
    const BY_LANGUAGE: &'static str = include_str!("by_language.sparql");

    async fn list(client: &Client) -> Result<Vec<Self>> {
        let query = Self::LIST;
        Ok(client.query::<Self>(query).await?)
    }

    pub async fn by_language(client: &Client, language: WikidataQ) -> Result<Vec<Self>> {
        let query = &Self::BY_LANGUAGE.replace("$1", &language.as_str());
        Ok(client.query::<Self>(query).await?)
    }
}

async fn list_phonemes(State(client): State<Client>) -> Result<List> {
    let phonemes = Phoneme::list(&client).await?;
    Ok(List { phonemes })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn list_phonemes() {
        let client = Client::new();
        Phoneme::list(&client).await.unwrap();
    }

    #[tokio::test]
    async fn list_phonemes_english() {
        let client = Client::new();
        let english_phonemes = Phoneme::by_language(&client, WikidataQ(1860))
            .await
            .unwrap();
        assert!(english_phonemes.len() > 1);
    }
}
