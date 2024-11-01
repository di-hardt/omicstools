// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports

#[derive(Debug, Serialize, Deserialize)]
pub struct Offset {
    #[serde(rename = "@idRef")]
    pub id_ref: String,
    #[serde(rename = "$value")]
    pub value: usize,
}
