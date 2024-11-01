// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::scan_window::ScanWindow;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanWindowList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "scanWindow")]
    pub scan_windows: Vec<ScanWindow>,
}
