use askama::Template;

use crate::languages::Language;

#[derive(Template)]
#[template(path = "status/status.html")]
pub(super) struct MainStatus {
    pub(super) finished_languages: i32,
    pub(super) wikidata_languages: Vec<Language>,
}

impl MainStatus {
    fn wikidata_count(&self) -> usize {
        self.wikidata_languages.len()
    }

    fn wikidata_phoneme_count(&self) -> u32 {
        self.wikidata_languages
            .iter()
            .map(|l| l.phoneme_count.value.parse::<u32>().unwrap_or_default())
            .sum()
    }
}
