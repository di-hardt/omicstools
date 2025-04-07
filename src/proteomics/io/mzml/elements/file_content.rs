use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement};
use crate::build_cv_params_validator;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileContent {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for FileContent {
    fn validate(&self) -> Result<()> {
        for cv_param in &self.cv_params {
            cv_param.validate()?;
        }
        self.validate_cv_params(&self.cv_params, "fileContent")?;
        Ok(())
    }
}

build_cv_params_validator! {
    FileContent,
    [
    ],
    [
        "MS:1000524", // data file content
    ],
    [
        "MS:1000252", // spectrum representation
    ],
    [
    ]
}
