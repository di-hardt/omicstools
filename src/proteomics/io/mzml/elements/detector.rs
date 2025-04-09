use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement};
use crate::build_cv_params_validator;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Detector {
    #[serde(rename = "@order")]
    pub order: usize,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for Detector {
    fn validate(&self) -> Result<()> {
        for cv_param in &self.cv_params {
            cv_param.validate()?;
        }
        self.validate_cv_params(&self.cv_params, "detector")?;
        Ok(())
    }
}

build_cv_params_validator! {
    Detector,
    [
        "MS:1000026", // detector type
    ],
    [],
    [],
    [
        "MS:1000481", // detector attribute
        "MS:1000027", // detector acquisition mode
    ]
}
