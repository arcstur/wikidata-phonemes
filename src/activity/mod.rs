mod templates;

use axum::routing::get;

use crate::AppRouter;
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

const OBELEPEDIA: [Language; 1] = [Language {
    qid: "Q21117",
    en_label: "Central Alaskan Yup'ik",
    pt_wikipedia: "https://pt.wikipedia.org/wiki/L%C3%ADngua_i%C3%BApique_do_Alasca_Central",
}];
