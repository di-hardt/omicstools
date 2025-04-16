use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::is_element::IsElement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserParam {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl IsElement for UserParam {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
