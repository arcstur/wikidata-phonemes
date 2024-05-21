use askama::Template;

use super::Language;

#[derive(Template)]
#[template(path = "languages/list.html")]
pub(super) struct List {
    pub(super) languages: Vec<Language>,
}
