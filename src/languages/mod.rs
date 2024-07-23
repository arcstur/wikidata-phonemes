mod templates;

use axum::{
    extract::{Form, Path, State},
    routing::{get, post},
};
use axum_login::AuthUser;
use sqlx::SqlitePool;

use crate::{
    api::{AddPhonemeInput, Editor},
    phonemes::Phoneme,
    AppRouter, Client, EntityId, Result, User, WikiValue,
};
use serde::Deserialize;
use templates::{AvailablePhonemes, List, PhonemeAdded, Status};

use self::templates::Details;

pub fn router() -> AppRouter {
    AppRouter::new()
        .route("/", get(list_languages))
        .route("/:id", get(single_language))
        .route("/:id/mark_as_working", post(mark_as_working))
        .route("/:id/unmark_as_working", post(unmark_as_working))
        .route("/:id/finish", post(finish))
        .route("/:id/undo_finish", post(undo_finish))
        .route("/:id/available_phonemes", get(available_phonemes))
        .route("/:id/status", get(status))
        .route("/:id/add_phoneme", post(add_phoneme))
}

async fn list_languages(State(client): State<Client>) -> Result<List> {
    let languages = Language::list(&client).await?;
    Ok(List { languages })
}

async fn single_language(State(client): State<Client>, Path(id): Path<String>) -> Result<Details> {
    let id = EntityId(id);
    let phonemes = Phoneme::by_language(&client, &id).await?;
    let label_or_id = client
        .english_label(&id)
        .await?
        .unwrap_or(format!("Language {id}"));

    Ok(Details {
        phonemes,
        label_or_id,
        id,
    })
}

#[derive(Deserialize)]
struct PhonemeForm {
    phoneme: String,
    wikipedia_url: String,
}

async fn add_phoneme(
    State(client): State<Client>,
    user: User,
    Path(id): Path<String>,
    Form(form): Form<PhonemeForm>,
) -> Result<PhonemeAdded> {
    let PhonemeForm {
        phoneme,
        wikipedia_url,
    } = form;

    let language = EntityId(id);
    let phoneme = EntityId(phoneme);

    let editor = Editor::new(&client, &user);
    editor
        .add_phoneme(AddPhonemeInput {
            language,
            phoneme,
            wikipedia_url,
        })
        .await?;

    Ok(PhonemeAdded {})
}

async fn available_phonemes(
    State(client): State<Client>,
    _user: User,
    Path(id): Path<String>,
) -> Result<AvailablePhonemes> {
    let id = EntityId(id);
    let label_or_id = client
        .english_label(&id)
        .await?
        .unwrap_or(format!("Language {id}"));
    let available_phonemes = Phoneme::by_language_opposite(&client, &id).await?;

    Ok(AvailablePhonemes {
        id,
        available_phonemes,
        label_or_id,
    })
}

async fn status(
    State(pool): State<SqlitePool>,
    user: Option<User>,
    Path(qid): Path<String>,
) -> Result<Status> {
    Status::generate(&pool, user, qid).await
}

impl Status {
    async fn generate(pool: &SqlitePool, user: Option<User>, qid: String) -> Result<Self> {
        let status = sqlx::query_as!(
            StatusQuery,
            "
            SELECT
                s.is_finished,
                u.username
            FROM language_status s
            LEFT JOIN users u ON (u.id = s.working_user)
            WHERE s.qid = ?
            ",
            qid,
        )
        .fetch_optional(pool)
        .await?
        .unwrap_or_default();

        let id = EntityId(qid);
        let working_user_username = status.username.unwrap_or_default();
        let is_finished = status.is_finished;

        Ok(Status {
            user,
            id,
            is_finished,
            working_user_username,
        })
    }
}

#[derive(Default)]
struct StatusQuery {
    is_finished: bool,
    username: Option<String>,
}

async fn mark_as_working(
    State(pool): State<SqlitePool>,
    user: User,
    Path(qid): Path<String>,
) -> Result<Status> {
    let user_id = user.id();

    sqlx::query!(
        "REPLACE INTO language_status ('qid', 'working_user') VALUES (?, ?)",
        qid,
        user_id,
    )
    .execute(&pool)
    .await?;

    Status::generate(&pool, Some(user), qid).await
}

async fn unmark_as_working(
    State(pool): State<SqlitePool>,
    user: User,
    Path(qid): Path<String>,
) -> Result<Status> {
    sqlx::query!("DELETE FROM language_status WHERE qid = ?", qid)
        .execute(&pool)
        .await?;
    Status::generate(&pool, Some(user), qid).await
}

async fn finish(
    user: User,
    State(pool): State<SqlitePool>,
    Path(qid): Path<String>,
) -> Result<Status> {
    sqlx::query!(
        "UPDATE language_status SET is_finished = true WHERE qid = ?",
        qid,
    )
    .execute(&pool)
    .await?;
    Status::generate(&pool, Some(user), qid).await
}

async fn undo_finish(
    user: User,
    State(pool): State<SqlitePool>,
    Path(qid): Path<String>,
) -> Result<Status> {
    sqlx::query!(
        "UPDATE language_status SET is_finished = false WHERE qid = ?",
        qid,
    )
    .execute(&pool)
    .await?;
    Status::generate(&pool, Some(user), qid).await
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
