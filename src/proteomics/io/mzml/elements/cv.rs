use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::is_element::IsElement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cv {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@fullName")]
    pub full_name: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@URI")]
    pub uri: String,
}

impl IsElement for Cv {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
