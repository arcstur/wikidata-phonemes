mod templates;

use axum::{extract::State, routing::get};

use crate::{AppRouter, Client, Result, WikiValue};
use serde::Deserialize;
use templates::List;

pub fn router() -> AppRouter {
    AppRouter::new().route("/", get(list_phonemes))
}

#[derive(Debug, Deserialize)]
struct Phoneme {
    #[serde(rename = "phoneme")]
    uri: WikiValue<String>,
    #[serde(rename = "phonemeLabel")]
    label: WikiValue<String>,
    transcriptions: WikiValue<String>,
    audio: Option<WikiValue<String>>,
}

impl Phoneme {
    const LIST: &'static str = include_str!("list.sparql");

    async fn list(client: &Client) -> Result<Vec<Self>> {
        let query = Self::LIST;
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
}
