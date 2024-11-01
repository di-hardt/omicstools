// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::index::Index;

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "index")]
    pub indexes: Vec<Index>,
}
