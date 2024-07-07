use serde::Serialize;
use tracing::instrument;

use super::Client;
use crate::{auth::User, EntityId, Result};

#[derive(Debug)]
pub struct Editor<'a> {
    client: &'a Client,
    user: &'a User,
}

impl<'a> Editor<'a> {
    const WIKIDATA_REST: &'static str = "https://www.wikidata.org/w/rest.php/wikibase/v0";
    const P_HAS_PHONEME: &'static str = "P2587";
    const P_WIKIMEDIA_IMPORT_URL: &'static str = "P4656";

    pub fn new(client: &'a Client, user: &'a User) -> Self {
        Self { client, user }
    }

    #[instrument(level = "info", ret, err)]
    pub async fn add_phoneme(&self, info: AddPhonemeInput) -> Result<()> {
        let endpoint = format!(
            "{}/entities/items/{}/statements",
            Self::WIKIDATA_REST,
            info.language.as_str()
        );

        let body = AddPhonemeBody {
            statement: Statement {
                property: Property {
                    id: EntityId::from(Self::P_HAS_PHONEME),
                },
                value: Value::Value {
                    content: info.phoneme,
                },
                references: vec![Reference::from_wikipedia_url(info.wikipedia_url)],
            },
            comment: String::from("Adding a phoneme."),
            bot: false,
        };

        self.client
            .inner
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.user.token()))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct AddPhonemeInput {
    pub language: EntityId,
    pub phoneme: EntityId,
    pub wikipedia_url: String,
}

#[derive(Serialize)]
struct AddPhonemeBody {
    statement: Statement,
    comment: String,
    bot: bool,
}

#[derive(Serialize)]
struct Statement {
    property: Property,
    value: Value,
    references: Vec<Reference>,
}

#[derive(Serialize)]
struct Property {
    id: EntityId,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum Value {
    Value { content: EntityId },
}

#[derive(Serialize)]
struct Reference {
    parts: Vec<ReferenceItem>,
}

#[derive(Serialize)]
struct ReferenceItem {
    property: Property,
    value: Value,
}

impl Reference {
    fn from_wikipedia_url(url: String) -> Self {
        Reference {
            parts: vec![ReferenceItem {
                property: Property {
                    id: EntityId::from(Editor::P_WIKIMEDIA_IMPORT_URL),
                },
                value: Value::Value {
                    content: EntityId(url),
                },
            }],
        }
    }
}
