use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::is_element::IsElement;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserParam {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
}

impl IsElement for UserParam {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
