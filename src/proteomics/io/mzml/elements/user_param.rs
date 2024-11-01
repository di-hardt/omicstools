// 3rd party imports
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserParam {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
}
