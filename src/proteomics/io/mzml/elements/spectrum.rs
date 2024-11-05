// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::{
    binary_data_array_list::BinaryDataArrayList, cv_param::CvParam, precursor_list::PrecursorList,
    scan_list::ScanList,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Spectrum {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@defaultArrayLength")]
    pub default_array_length: String,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(rename = "scanList")]
    pub scan_list: ScanList,
    #[serde(rename = "precursorList")]
    pub precursor_list: Option<PrecursorList>,
    #[serde(rename = "binaryDataArrayList")]
    pub binary_data_array_list: Vec<BinaryDataArrayList>,
}
