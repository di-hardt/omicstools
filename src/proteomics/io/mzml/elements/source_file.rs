use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement};
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

impl IsElement for SourceFile {
    fn validate(&self) -> Result<()> {
        for cv_param in &self.cv_params {
            cv_param.validate()?;
        }
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
    [],
    []
);
