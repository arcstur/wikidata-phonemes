mod templates;

use axum::{
    extract::{Form, Path, State},
    response::IntoResponse,
    routing::{get, post},
};

use crate::{
    api::{AddPhoneme, EditingClient},
    phonemes::Phoneme,
    AppRouter, Client, EntityId, Result, User, WikiValue,
};
use serde::Deserialize;
use templates::List;

use self::templates::Details;

pub fn router() -> AppRouter {
    AppRouter::new()
        .route("/", get(list_languages))
        .route("/:id", get(single_language))
        .route("/:id/add_phoneme", post(add_phoneme))
}

async fn list_languages(State(client): State<Client>) -> Result<List> {
    let languages = Language::list(&client).await?;
    Ok(List { languages })
}

async fn single_language(
    user: Option<User>,
    State(client): State<Client>,
    Path(id): Path<String>,
) -> Result<Details> {
    let id = EntityId(id);
    let phonemes = Phoneme::by_language(&client, &id).await?;
    let available_phonemes = Phoneme::by_language_opposite(&client, &id).await?;
    let label_or_id = client
        .english_label(&id)
        .await?
        .unwrap_or(format!("Language {id}"));
    let is_logged_in = user.is_some();
    Ok(Details {
        is_logged_in,
        phonemes,
        available_phonemes,
        label_or_id,
        id,
    })
}

#[derive(Deserialize)]
struct PhonemeForm {
    phoneme: String,
    reference_url: Option<String>,
}

async fn add_phoneme(
    State(client): State<Client>,
    user: User,
    Path(id): Path<String>,
    Form(form): Form<PhonemeForm>,
) -> Result<axum::response::Response> {
    let language = EntityId(id);
    let phoneme = EntityId(form.phoneme);

    let editing = EditingClient::new(&client, &user);

    // editing
    //     .add_phoneme(AddPhoneme { language, phoneme })
    //     .await?;

    dbg!("would add phoneme:", AddPhoneme { language, phoneme });

    Ok(String::new().into_response())
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
