use askama::Template;

use super::Language;
use crate::Phoneme;

#[derive(Template)]
#[template(path = "languages/list.html")]
pub(super) struct List {
    pub(super) languages: Vec<Language>,
}

#[derive(Template)]
#[template(path = "languages/details.html")]
pub(super) struct Details {
    pub(super) phonemes: Vec<Phoneme>,
}
