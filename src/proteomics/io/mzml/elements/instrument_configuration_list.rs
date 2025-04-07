use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{instrument_configuration::InstrumentConfiguration, is_element::IsElement};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentConfigurationList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "instrumentConfiguration")]
    pub instrument_configurations: Vec<InstrumentConfiguration>,
}

impl IsElement for InstrumentConfigurationList {
    fn validate(&self) -> Result<()> {
        for instrument_configuration in &self.instrument_configurations {
            instrument_configuration.validate()?;
        }
        Ok(())
    }
}
