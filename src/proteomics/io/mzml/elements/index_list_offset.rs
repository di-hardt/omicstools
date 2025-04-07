use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::is_element::IsElement;

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexListOffset {
    #[serde(rename = "$value")]
    pub value: usize,
}
impl IsElement for IndexListOffset {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
