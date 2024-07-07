use askama::Template;

use super::Language;
use crate::{EntityId, Phoneme};

#[derive(Template)]
#[template(path = "languages/list.html")]
pub(super) struct List {
    pub(super) languages: Vec<Language>,
}

impl List {
    fn language_count(&self) -> usize {
        self.languages.iter().count()
    }

    fn total_phoneme_count(&self) -> u32 {
        self.languages
            .iter()
            .map(|l| l.phoneme_count.value.parse::<u32>().unwrap_or_default())
            .sum()
    }
}

#[derive(Template)]
#[template(path = "languages/details.html")]
pub(super) struct Details {
    pub(super) is_logged_in: bool,
    pub(super) phonemes: Vec<Phoneme>,
    pub(super) available_phonemes: Vec<Phoneme>,
    pub(super) label_or_id: String,
    pub(super) id: EntityId,
}
