use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{is_element::IsElement, offset::Offset};

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default, rename = "offset")]
    pub offsets: Vec<Offset>,
}

impl IsElement for Index {
    fn validate(&self) -> Result<()> {
        for offset in &self.offsets {
            offset.validate()?;
        }
        Ok(())
    }
}
