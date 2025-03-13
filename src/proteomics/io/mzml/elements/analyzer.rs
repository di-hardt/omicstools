use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    cv_param::CvParam, referenceable_param_group_ref::ReferenceableParamGroupRef,
    user_param::UserParam,
};
use crate::build_cv_params_validator;

#[derive(Debug, Serialize, Deserialize)]
pub struct Analyzer {
    #[serde(rename = "@order")]
    pub order: usize,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "referenceableParamGroupRef")]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}

impl Analyzer {
    pub fn validate(&self) -> Result<()> {
        self.validate_cv_params(&self.cv_params, "activation")?;
        Ok(())
    }
}

build_cv_params_validator! {
    Analyzer,
    [
        "MS:1000480", // mass analyzer attribute
    ],
    [
        "MS:1000443", // mass analyzer type
    ],
    [
    ]
}
