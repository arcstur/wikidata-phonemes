mod templates;

use axum::{extract::State, routing::get};

use self::templates::MainStatus;
use crate::{app::AppState, languages::Language, AppRouter, Result};

pub fn router() -> AppRouter {
    AppRouter::new().route("/", get(main_status))
}

async fn main_status(State(AppState { client, pool }): State<AppState>) -> Result<MainStatus> {
    let finished_languages =
        sqlx::query!("SELECT SUM(is_finished) AS finished FROM language_status")
            .fetch_one(&pool)
            .await?
            .finished
            .unwrap_or(0);

    let wikidata_languages = Language::list(&client).await?;

    Ok(MainStatus {
        finished_languages,
        wikidata_languages,
    })
}
