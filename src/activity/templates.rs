use super::Language;
use askama::Template;

#[derive(Template)]
#[template(path = "activity/list.html")]
pub(super) struct List {
    pub(super) obelepedia_languages: Vec<Language>,
    pub(super) working_languages: Vec<Language>,
}
