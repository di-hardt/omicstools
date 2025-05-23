use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{
    is_element::IsElement,
    is_list::IsList,
    scan_window::ScanWindow,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl IsList<'_, ScanWindow> for ScanWindowList {
    fn iter(&self) -> std::slice::Iter<'_, ScanWindow> {
        self.scan_windows.iter()
    }
}
