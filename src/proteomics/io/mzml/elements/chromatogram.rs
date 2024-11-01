// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::{binary_data_array_list::BinaryDataArrayList, cv_param::CvParam};

#[derive(Debug, Serialize, Deserialize)]
pub struct Chromatogram {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@defaultArrayLength")]
    pub default_array_length: String,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(rename = "binaryDataArrayList")]
    pub binary_data_array_list: Vec<BinaryDataArrayList>,
}
