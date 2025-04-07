use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{is_element::IsElement, scan_window::ScanWindow};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanWindowList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "scanWindow")]
    pub scan_windows: Vec<ScanWindow>,
}

impl IsElement for ScanWindowList {
    fn validate(&self) -> Result<()> {
        if self.count != self.scan_windows.len() {
            bail!("ScanWindowList count does not match the number of scan windows");
        }
        for scan_window in &self.scan_windows {
            scan_window.validate()?;
        }
        Ok(())
    }
}
