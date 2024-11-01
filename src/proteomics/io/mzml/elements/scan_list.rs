// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::{cv_param::CvParam, scan::Scan};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "scan")]
    pub scans: Vec<Scan>,
}
