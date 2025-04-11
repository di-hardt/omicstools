use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement};
use crate::has_cv_params;

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl IsElement for SourceFile {
    fn validate(&self) -> Result<()> {
        self.validate_cv_params("sourceFile")?;
        Ok(())
    }
}

has_cv_params!(
    SourceFile,
    cv_params,
    [
        "MS:1000767", // native spectrum identifier format
        "MS:1000560", // source file type
    ],
    [
        "MS:1000561" // data file checksum type
    ],
    [],
    []
);
