use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement};
use crate::build_cv_params_validator;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedIon {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for SelectedIon {
    fn validate(&self) -> Result<()> {
        for cv_param in &self.cv_params {
            cv_param.validate()?;
        }
        self.validate_cv_params(&self.cv_params, "selectedIon")?;
        Ok(())
    }
}

build_cv_params_validator! {
    SelectedIon,
    [],
    [
        "MS:1000455", // ion selection attribute
    ],
    [],
    []
}
