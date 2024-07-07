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

const OBELEPEDIA: [Language; 2] = [
    Language {
        qid: "Q21117",
        en_label: "Central Alaskan Yup'ik",
        pt_wikipedia: "https://pt.wikipedia.org/w/index.php?title=L%C3%ADngua_i%C3%BApique_do_Alasca_Central&oldid=67946266",
    },
    Language {
        qid: "Q4023175",
        en_label: "Jurúna",
        pt_wikipedia:
            "https://pt.wikipedia.org/w/index.php?title=L%C3%ADngua_yudj%C3%A1&oldid=67386586",
    },
];
