use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{is_element::IsElement, spectrum::Spectrum};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpectrumList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(rename = "@defaultDataProcessingRef")]
    pub default_data_processing_ref: String,
    #[serde(default, rename = "spectrum")]
    pub spectra: Vec<Spectrum>,
}

impl IsElement for SpectrumList {
    fn validate(&self) -> Result<()> {
        if self.count != self.spectra.len() {
            bail!("SpectrumList count does not match the number of spectra");
        }
        for spectrum in &self.spectra {
            spectrum.validate()?;
        }
        Ok(())
    }
}
