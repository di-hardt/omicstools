use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// Local imports
use super::{is_element::IsElement, is_list::IsList, software::Software};

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl IsList<'_, Software> for SoftwareList {
    fn iter(&self) -> std::slice::Iter<'_, Software> {
        self.softwares.iter()
    }
}
