// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::{binary::Binary, cv_param::CvParam};

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryDataArray {
    #[serde(rename = "@encodedLength")]
    pub encoded_length: usize,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(rename = "binary")]
    pub binary: Binary,
}
