// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::chromatogram::Chromatogram;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChromatogramList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(rename = "@defaultDataProcessingRef")]
    pub default_data_processing_ref: String,
    #[serde(default, rename = "chromatogram")]
    pub chromatograms: Vec<Chromatogram>,
}
