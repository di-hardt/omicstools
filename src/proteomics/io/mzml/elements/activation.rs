use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, referenceable_param_group_ref::ReferenceableParamGroupRef};
use crate::build_cv_params_validator;

#[derive(Debug, Serialize, Deserialize)]

pub struct Activation {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "referenceableParamGroupRef")]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
}

impl Activation {
    pub fn validate(&self) -> Result<()> {
        self.validate_cv_params(&self.cv_params, "activation")?;
        Ok(())
    }
}

build_cv_params_validator! {
    Activation,
    [],
    [
        "MS:1000044", // dissociation method
        ""
    ],
    [
        "MS:1000510", // precursor activation attribute
    ]
}
