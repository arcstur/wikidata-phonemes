mod templates;

use axum::{extract::State, routing::get};
use sqlx::SqlitePool;

use crate::{AppRouter, Result};
use templates::List;

pub fn router() -> AppRouter {
    AppRouter::new().route("/", get(languages_list))
}

async fn languages_list(State(pool): State<SqlitePool>) -> Result<List> {
    let languages = sqlx::query!(
        "
        SELECT
            l.qid,
            l.en_label,
            l.wikipedia_url,
            s.is_finished,
            COALESCE(wu.username, '') AS working_user_username
        FROM languages l
        LEFT JOIN language_status s ON (s.qid = l.qid)
        LEFT JOIN users wu ON (s.working_user = wu.id)
        "
    )
    .fetch_all(&pool)
    .await?
    .into_iter()
    .map(|r| Language {
        qid: r.qid,
        en_label: r.en_label,
        wikipedia_url: r.wikipedia_url,
        is_finished: r.is_finished.unwrap_or(false),
        working_user_username: r.working_user_username,
    })
    .collect();

    Ok(List { languages })
}

struct Language {
    qid: String,
    en_label: String,
    wikipedia_url: String,
    is_finished: bool,
    working_user_username: String,
}
