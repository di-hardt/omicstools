use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::build_cv_params_validator;

// Local imports
use super::{cv_param::CvParam, is_element::IsElement};

#[derive(Debug, Serialize, Deserialize)]
pub struct IsolationWindow {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for IsolationWindow {
    fn validate(&self) -> Result<()> {
        for cv_params in &self.cv_params {
            cv_params.validate()?;
        }
        self.validate_cv_params(&self.cv_params, "activation")?;
        Ok(())
    }
}

build_cv_params_validator! {
    IsolationWindow,
    [
    ],
    [
        "MS:1000792", // isolation window attribute
    ],
    [],
    [
    ]
}
