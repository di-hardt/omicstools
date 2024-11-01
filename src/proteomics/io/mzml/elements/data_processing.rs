// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::processing_method::ProcessingMethod;

#[derive(Debug, Serialize, Deserialize)]
pub struct DataProcessing {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(default, rename = "processingMethod")]
    pub processing_methods: Vec<ProcessingMethod>,
}
