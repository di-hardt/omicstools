// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::software::Software;

#[derive(Debug, Serialize, Deserialize)]
pub struct SoftwareList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "software")]
    pub softwares: Vec<Software>,
}
