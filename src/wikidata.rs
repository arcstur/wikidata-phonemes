use std::fmt::Display;

use serde::Deserialize;

pub struct WikidataQ(pub u32);

impl WikidataQ {
    pub fn as_str(&self) -> String {
        format!("Q{}", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct Uri {
    #[serde(rename = "type")]
    kind: String,
    value: String,
}

impl Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<Uri> for WikidataQ {
    type Error = Box<dyn std::error::Error>;

    fn try_from(uri: Uri) -> Result<Self, Self::Error> {
        let value = uri.value;
        let (_, id) = value
            .rsplit_once("/entity/Q")
            .ok_or("No /entity/Q found in URI.")?;
        let id = id.parse::<u32>()?;
        Ok(WikidataQ(id))
    }
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
