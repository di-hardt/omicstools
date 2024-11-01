// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::cv_param::CvParam;

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceFile {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@location")]
    pub location: String,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}
