use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// Local imports
use super::{is_element::IsElement, software::Software};

#[derive(Debug, Serialize, Deserialize)]
pub struct SoftwareList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "software")]
    pub softwares: Vec<Software>,
}

impl IsElement for SoftwareList {
    fn validate(&self) -> Result<()> {
        if self.count != self.softwares.len() {
            bail!("SoftwareList count does not match the number of software elements");
        }
        for software in &self.softwares {
            software.validate()?;
        }
        Ok(())
    }
}
