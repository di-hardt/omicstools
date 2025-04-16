use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::is_element::IsElement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Offset {
    #[serde(rename = "@idRef")]
    pub id_ref: String,
    #[serde(rename = "$value")]
    pub value: usize,
}

impl IsElement for Offset {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
