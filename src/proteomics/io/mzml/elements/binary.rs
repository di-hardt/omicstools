use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::is_element::IsElement;

#[derive(Debug, Serialize, Deserialize)]
pub struct Binary {
    #[serde(rename = "$value")]
    pub data: String,
}

impl IsElement for Binary {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
