use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    cv_param::CvParam, is_element::IsElement,
    referenceable_param_group_ref::ReferenceableParamGroupRef, user_param::UserParam,
};
use crate::has_cv_params;

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl IsElement for Analyzer {
    fn validate(&self) -> Result<()> {
        self.validate_cv_params("analyzer")?;
        Ok(())
    }
}

has_cv_params! {
    Analyzer,
    cv_params,
    [
        "MS:1000443", // mass analyzer type
    ],
    [],
    [],
    [
        "MS:1000480", // mass analyzer attribute
    ]
}
