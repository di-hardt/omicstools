use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    chromatogram_list::ChromatogramList, is_element::IsElement, spectrum_list::SpectrumList,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Run {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@defaultInstrumentConfigurationRef")]
    pub default_instrument_configuration_ref: String,
    #[serde(rename = "@startTimeStamp")]
    pub start_time_stamp: String,
    #[serde(rename = "@defaultSourceFileRef")]
    pub default_source_file_ref: String,
    #[serde(rename = "spectrumList")]
    pub spectrum_list: SpectrumList,
    #[serde(rename = "chromatogramList")]
    pub chromatogram_list: ChromatogramList,
}

impl IsElement for Run {
    fn validate(&self) -> Result<()> {
        self.spectrum_list.validate()?;
        self.chromatogram_list.validate()?;
        Ok(())
    }
}

/// Implementation of the MzML element <run> without spectrum and chromatogram data.
/// This is useful for indexing the MzML file.
///
#[derive(Debug, Deserialize)]
pub struct IndexedRun {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@defaultInstrumentConfigurationRef")]
    pub default_instrument_configuration_ref: String,
    #[serde(rename = "@startTimeStamp")]
    pub start_time_stamp: String,
    #[serde(rename = "@defaultSourceFileRef")]
    pub default_source_file_ref: String,
}

impl IsElement for IndexedRun {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
