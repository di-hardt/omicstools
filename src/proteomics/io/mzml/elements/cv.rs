// 3rd party imports
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cv {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@fullName")]
    pub full_name: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@URI")]
    pub uri: String,
}
