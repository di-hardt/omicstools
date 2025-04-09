use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement};
use crate::build_cv_params_validator;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanWindow {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for ScanWindow {
    fn validate(&self) -> Result<()> {
        for cv_param in &self.cv_params {
            cv_param.validate()?;
        }
        self.validate_cv_params(&self.cv_params, "ScanWindow")?;
        Ok(())
    }
}

build_cv_params_validator! {
    ScanWindow,
    [
        "MS:1000500", // scan window upper limit
        "MS:1000501", // scan window lower limit
    ],
    [
        "MS:1000549", // selection window attribute
    ],
    [],
    []
}
