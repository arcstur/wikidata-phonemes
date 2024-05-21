use askama::Template;

use super::Phoneme;

#[derive(Template)]
#[template(path = "phonemes/list.html")]
pub(super) struct List {
    pub(super) phonemes: Vec<Phoneme>,
}
