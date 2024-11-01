// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::{chromatogram_list::ChromatogramList, spectrum_list::SpectrumList};

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
