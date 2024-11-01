// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::precursor::Precursor;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrecursorList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "precursor")]
    pub precursors: Vec<Precursor>,
}
