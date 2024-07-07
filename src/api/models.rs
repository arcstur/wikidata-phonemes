use std::fmt::Display;

use serde::{de::Error, Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityId(#[serde(deserialize_with = "from_uri")] pub String);

impl EntityId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://www.wikidata.org/entity/{}", self.0)
    }
}

fn from_uri<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let uri: WikiValue<String> = Deserialize::deserialize(deserializer)?;
    let s = uri.value;

    let (_, id) = s
        .rsplit_once("/entity/")
        .ok_or(D::Error::custom(format!("no /entity/ in URI: {}", s)))?;

    Ok(id.to_owned())
}

#[derive(Debug, Deserialize)]
pub struct WikiValue<T> {
    #[serde(rename = "type")]
    pub kind: String,
    pub value: T,
}

impl<T: Display> Display for WikiValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
