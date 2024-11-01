// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::binary_data_array::BinaryDataArray;

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryDataArrayList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "binaryDataArray")]
    pub binary_data_arrays: Vec<BinaryDataArray>,
}
