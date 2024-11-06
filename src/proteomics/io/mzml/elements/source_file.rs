use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::cv_param::CvParam;
use crate::build_cv_params_validator;

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

impl SourceFile {
    pub fn validate(&self) -> Result<()> {
        self.validate_cv_params(&self.cv_params, "sourceFile")?;
        Ok(())
    }
}

build_cv_params_validator!(
    SourceFile,
    [
        "MS:1000767", // native spectrum identifier format
        "MS:1000560", // source file type
    ],
    [
        "MS:1000561" // data file checksum type
    ],
    []
);
