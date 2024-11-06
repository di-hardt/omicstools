use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::precursor::Precursor;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrecursorList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "precursor")]
    pub precursors: Vec<Precursor>,
}

impl PrecursorList {
    pub fn validate(&self) -> Result<()> {
        for precursor in &self.precursors {
            precursor.validate()?;
        }
        Ok(())
    }
}
