use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement, user_param::UserParam};
use crate::build_cv_params_validator;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessingMethod {
    #[serde(rename = "@order")]
    pub order: usize,
    #[serde(rename = "@softwareRef")]
    pub software_ref: String,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl IsElement for ProcessingMethod {
    fn validate(&self) -> Result<()> {
        for cv_param in &self.cv_params {
            cv_param.validate()?;
        }
        for user_param in &self.user_params {
            user_param.validate()?;
        }
        self.validate_cv_params(&self.cv_params, "processingMethod")?;
        Ok(())
    }
}

build_cv_params_validator! {
    ProcessingMethod,
    [],
    [
        "MS:1000452", // data transformation
    ],
    [],
    [
        "MS:1000630" //data processing parameter
    ]
}
