mod obelepedia;
mod templates;

use axum::routing::get;

use crate::AppRouter;
use obelepedia::OBELEPEDIA;
use templates::List;

pub fn router() -> AppRouter {
    AppRouter::new().route("/", get(obelepedia_list))
}

async fn obelepedia_list() -> List {
    let languages = OBELEPEDIA.into();
    List { languages }
}

struct Language {
    qid: &'static str,
    en_label: &'static str,
    pt_wikipedia: &'static str,
}
