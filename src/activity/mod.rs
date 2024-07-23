mod obelepedia;
mod templates;
mod working_languages;

use axum::routing::get;
use working_languages::WORKING_LANGUAGES;

use crate::AppRouter;
use obelepedia::OBELEPEDIA;
use templates::List;

pub fn router() -> AppRouter {
    AppRouter::new().route("/", get(languages_list))
}

async fn languages_list() -> List {
    let obelepedia_languages = OBELEPEDIA.into();
    let working_languages = WORKING_LANGUAGES.into();
    List {
        obelepedia_languages,
        working_languages,
    }
}

struct Language {
    qid: &'static str,
    en_label: &'static str,
    wikipedia_url: &'static str,
}
