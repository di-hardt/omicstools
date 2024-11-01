// 3rd party imports
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CvParam {
    #[serde(rename = "@cvRef")]
    pub cv_ref: String,
    #[serde(rename = "@accession")]
    pub accession: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "@unitCvRef")]
    pub unit_cv_ref: Option<String>,
    #[serde(rename = "@unitAccession")]
    pub unit_accession: Option<String>,
    #[serde(rename = "@unitName")]
    pub unit_name: Option<String>,
}
