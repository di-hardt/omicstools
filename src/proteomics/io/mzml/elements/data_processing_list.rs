// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::data_processing::DataProcessing;

#[derive(Debug, Serialize, Deserialize)]
pub struct DataProcessingList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "dataProcessing")]
    pub data_processings: Vec<DataProcessing>,
}
