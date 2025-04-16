use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{
    is_element::IsElement,
    is_list::IsList,
    spectrum::Spectrum,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

impl IsList<'_, Spectrum> for SpectrumList {
    fn iter(&self) -> std::slice::Iter<'_, Spectrum> {
        self.spectra.iter()
    }
}
