// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::instrument_configuration::InstrumentConfiguration;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentConfigurationList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "instrumentConfiguration")]
    pub instrument_configurations: Vec<InstrumentConfiguration>,
}
