use serde::Serialize;

use super::Client;
use crate::{auth::User, Result, WikidataQ};

struct EditingClient<'a> {
    client: &'a Client,
    user: &'a User,
}

impl<'a> EditingClient<'a> {
    const WIKIDATA_REST: &'static str = "https://www.wikidata.org/w/rest.php/wikibase/v0";
    const P_HAS_PHONEME: &'static str = "P2587";

    pub async fn add_phoneme(&self, info: AddPhoneme) -> Result<()> {
        let endpoint = format!(
            "{}/entities/items/{}/statements",
            Self::WIKIDATA_REST,
            info.language.as_str()
        );

        let body = AddPhonemeBody {
            statement: Statement {
                property: Property {
                    id: Self::P_HAS_PHONEME,
                },
                value: Value::Value {
                    content: info.phoneme.as_str(),
                },
            },
            comment: String::from("Testing out the API..."),
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

pub struct AddPhoneme {
    pub language: WikidataQ,
    pub phoneme: WikidataQ,
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
}

#[derive(Serialize)]
struct Property {
    id: &'static str,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum Value {
    Value { content: String },
}
