use std::fmt::Display;

use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct WikidataQ(#[serde(deserialize_with = "from_q_uri")] pub u32);

impl WikidataQ {
    pub fn as_str(&self) -> String {
        format!("Q{}", self.0)
    }
}

impl Display for WikidataQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://www.wikidata.org/entity/Q{}", self.0)
    }
}

fn from_q_uri<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let uri: WikiValue<String> = Deserialize::deserialize(deserializer)?;
    let s = uri.value;

    let (_, id) = s
        .rsplit_once("/entity/Q")
        .ok_or(D::Error::custom(format!("no /entity/Q in URI: {}", s)))?;

    id.parse::<u32>().map_err(D::Error::custom)
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
