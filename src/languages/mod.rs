mod templates;

use axum::{
    extract::{Form, Path, State},
    response::Redirect,
    routing::{get, post},
};
use axum_login::AuthUser;
use sqlx::SqlitePool;

use crate::{
    api::{AddPhonemeInput, Editor},
    app::AppState,
    phonemes::Phoneme,
    AppRouter, Client, EntityId, Error, Result, User, WikiValue,
};
use serde::Deserialize;
use templates::{AvailablePhonemes, PhonemeAdded, Status, UnmarkAsWorkingButton};

use self::templates::Details;

pub fn router() -> AppRouter {
    AppRouter::new()
        .route("/", get(go_to_status))
        .route("/:id", get(single_language))
        .route("/:id/mark_as_working", post(mark_as_working))
        .route("/:id/unmark_as_working", post(unmark_as_working))
        .route("/:id/dev", get(unmark_as_working_button))
        .route("/:id/finish", post(finish))
        .route("/:id/undo_finish", post(undo_finish))
        .route("/:id/available_phonemes", get(available_phonemes))
        .route("/:id/status", get(status))
        .route("/:id/add_phoneme", post(add_phoneme))
}

async fn go_to_status() -> Redirect {
    Redirect::to("/status")
}

async fn single_language(
    user: Option<User>,
    State(state): State<AppState>,
    Path(qid): Path<String>,
) -> Result<Details> {
    let client = state.client;
    let pool = state.pool;

    let id = EntityId(qid.clone());

    let row = sqlx::query!(
        "
        SELECT en_label, wikipedia_url
        FROM languages
        WHERE qid = ?
        ",
        qid,
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(Error::LanguageNotInList)?;

    let phonemes = Phoneme::by_language(&client, &id).await?;

    Ok(Details {
        phonemes,
        id,
        en_label: row.en_label,
        wikipedia_url: row.wikipedia_url,
        status: Status::generate(&pool, user, qid).await?,
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
    State(state): State<AppState>,
    _user: User,
    Path(qid): Path<String>,
) -> Result<AvailablePhonemes> {
    let pool = state.pool;
    let client = state.client;

    let en_label = sqlx::query!("SELECT en_label FROM languages WHERE qid = ?", qid)
        .fetch_one(&pool)
        .await?
        .en_label;

    let id = EntityId(qid);
    let available_phonemes = Phoneme::by_language_opposite(&client, &id).await?;

    Ok(AvailablePhonemes {
        id,
        available_phonemes,
        en_label,
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

    let working_user_exists = sqlx::query!(
        "SELECT working_user FROM language_status WHERE qid = ?",
        qid
    )
    .fetch_optional(&pool)
    .await?
    .is_some_and(|row| row.working_user.is_some());

    if !working_user_exists {
        sqlx::query!(
            "REPLACE INTO language_status ('qid', 'working_user') VALUES (?, ?)",
            qid,
            user_id,
        )
        .execute(&pool)
        .await?;
    }

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

async fn unmark_as_working_button(_user: User, Path(qid): Path<String>) -> UnmarkAsWorkingButton {
    let id = EntityId(qid);
    UnmarkAsWorkingButton { id }
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
pub(super) struct Language {
    #[serde(rename = "language")]
    pub(super) q: EntityId,
    #[serde(rename = "languageLabel")]
    pub(super) label: WikiValue<String>,
    pub(super) phoneme_count: WikiValue<String>,
}

impl Language {
    const LIST: &'static str = include_str!("languages_phoneme_count.sparql");

    pub(super) async fn list(client: &Client) -> Result<Vec<Self>> {
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
