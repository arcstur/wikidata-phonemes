use super::Language;
use askama::Template;

#[derive(Template)]
#[template(path = "activity/list.html")]
pub(super) struct List {
    pub(super) languages: Vec<Language>,
}
