// 3rd party imports
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexListOffset {
    #[serde(rename = "$value")]
    pub value: usize,
}
