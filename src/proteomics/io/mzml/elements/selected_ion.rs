// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::cv_param::CvParam;

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectedIon {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}
