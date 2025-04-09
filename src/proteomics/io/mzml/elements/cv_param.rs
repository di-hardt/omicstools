use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::is_element::IsElement;

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
    #[serde(rename = "@unitCvRef", skip_serializing_if = "Option::is_none")]
    pub unit_cv_ref: Option<String>,
    #[serde(rename = "@unitAccession", skip_serializing_if = "Option::is_none")]
    pub unit_accession: Option<String>,
    #[serde(rename = "@unitName", skip_serializing_if = "Option::is_none")]
    pub unit_name: Option<String>,
}

impl IsElement for CvParam {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
