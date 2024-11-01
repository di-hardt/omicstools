// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::spectrum::Spectrum;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpectrumList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(rename = "@defaultDataProcessingRef")]
    pub default_data_processing_ref: String,
    #[serde(default, rename = "spectrum")]
    pub spectra: Vec<Spectrum>,
}
