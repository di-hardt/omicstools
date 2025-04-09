use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    chromatogram_list::ChromatogramList, is_element::IsElement, spectrum_list::SpectrumList,
};

pub trait IsRun: IsElement + Into<Run> {}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl IsRun for Run {}

// Necessary for separating spectra from an indexed run
impl From<IndexedRun> for Run {
    fn from(indexed_run: IndexedRun) -> Self {
        Self {
            id: indexed_run.id,
            default_instrument_configuration_ref: indexed_run.default_instrument_configuration_ref,
            start_time_stamp: indexed_run.start_time_stamp,
            default_source_file_ref: indexed_run.default_source_file_ref,
            spectrum_list: SpectrumList::default(),
            chromatogram_list: ChromatogramList::default(),
        }
    }
}

/// Implementation of the MzML element <run> without spectrum and chromatogram data.
/// This is useful for indexing the MzML file.
///
#[derive(Clone, Debug, Deserialize)]
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

impl IsRun for IndexedRun {}
