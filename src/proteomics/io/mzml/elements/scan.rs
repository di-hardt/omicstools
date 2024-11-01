// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::{cv_param::CvParam, scan_window_list::ScanWindowList, user_param::UserParam};

#[derive(Debug, Serialize, Deserialize)]
pub struct Scan {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
    #[serde(rename = "scanWindowList")]
    pub scan_window_list: ScanWindowList,
}
