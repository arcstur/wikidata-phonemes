use askama::Template;

use super::Language;
use crate::{EntityId, Phoneme, User};

#[derive(Template)]
#[template(path = "languages/list.html")]
pub(super) struct List {
    pub(super) languages: Vec<Language>,
}

impl List {
    fn language_count(&self) -> usize {
        self.languages.len()
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
    pub(super) phonemes: Vec<Phoneme>,
    pub(super) label_or_id: String,
    pub(super) id: EntityId,
    pub(super) status: Status,
}

#[derive(Template)]
#[template(path = "languages/status.html")]
pub(super) struct Status {
    pub(super) user: Option<User>,
    pub(super) id: EntityId,
    pub(super) is_finished: bool,
    pub(super) working_user_username: String,
}

#[derive(Template)]
#[template(path = "languages/available_phonemes.html")]
pub(super) struct AvailablePhonemes {
    pub(super) available_phonemes: Vec<Phoneme>,
    pub(super) id: EntityId,
    pub(super) label_or_id: String,
}

#[derive(Template)]
#[template(
    source = r#"<p style="margin-bottom: 3em"> Phoneme added successfully! </p>"#,
    ext = "html"
)]
pub(super) struct PhonemeAdded {}
