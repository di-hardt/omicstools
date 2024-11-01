// 3rd party imports
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Binary {
    #[serde(rename = "$value")]
    pub data: String,
}
