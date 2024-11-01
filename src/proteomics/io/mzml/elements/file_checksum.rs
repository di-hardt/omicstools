// 3rd party imports
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileChecksum {
    #[serde(rename = "$value")]
    pub value: String,
}
