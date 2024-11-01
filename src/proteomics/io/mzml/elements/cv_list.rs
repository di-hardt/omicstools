// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::cv::Cv;

#[derive(Debug, Serialize, Deserialize)]
pub struct CvList {
    #[serde(rename = "@count")]
    pub count: isize,
    #[serde(rename = "cv")]
    pub cv: Vec<Cv>,
}
