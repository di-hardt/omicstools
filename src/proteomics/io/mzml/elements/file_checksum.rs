use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::is_element::IsElement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileChecksum {
    #[serde(rename = "$value")]
    pub value: String,
}

impl IsElement for FileChecksum {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
