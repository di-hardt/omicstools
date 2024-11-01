// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::offset::Offset;

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default, rename = "offset")]
    pub offsets: Vec<Offset>,
}
