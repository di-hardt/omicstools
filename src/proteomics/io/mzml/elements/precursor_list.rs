use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{is_element::IsElement, is_list::IsList, precursor::Precursor};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrecursorList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "precursor")]
    pub precursors: Vec<Precursor>,
}

impl IsElement for PrecursorList {
    fn validate(&self) -> Result<()> {
        if self.count != self.precursors.len() {
            bail!("PrecursorList count does not match the number of precursors");
        }
        for precursor in &self.precursors {
            precursor.validate()?;
        }
        Ok(())
    }
}

impl IsList<'_, Precursor> for PrecursorList {
    fn iter(&self) -> std::slice::Iter<'_, Precursor> {
        self.precursors.iter()
    }
}
