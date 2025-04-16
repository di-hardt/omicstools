use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    chromatogram::Chromatogram,
    is_element::IsElement,
    is_list::IsList,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ChromatogramList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(rename = "@defaultDataProcessingRef")]
    pub default_data_processing_ref: String,
    #[serde(default, rename = "chromatogram")]
    pub chromatograms: Vec<Chromatogram>,
}

impl IsElement for ChromatogramList {
    fn validate(&self) -> Result<()> {
        for chromatogram in &self.chromatograms {
            chromatogram.validate()?;
        }
        Ok(())
    }
}

impl IsList<'_, Chromatogram> for ChromatogramList {
    fn iter(&self) -> std::slice::Iter<'_, Chromatogram> {
        self.chromatograms.iter()
    }
}
