use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement};
use crate::build_cv_params_validator;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "@order")]
    pub order: usize,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for Source {
    fn validate(&self) -> Result<()> {
        for cv_param in &self.cv_params {
            cv_param.validate()?;
        }
        self.validate_cv_params(&self.cv_params, "source")?;
        Ok(())
    }
}

build_cv_params_validator! {
    Source,
    [
        "MS:1000008", // ionization type
    ],
    [],
    [
        "MS:1000007", // inlet type
    ],
    [
        "MS:1000482", // source attribute
    ]
}
