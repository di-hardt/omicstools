use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    cv_param::CvParam, is_element::IsElement,
    referenceable_param_group_ref::ReferenceableParamGroupRef,
};
use crate::has_cv_params;

#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct Activation {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "referenceableParamGroupRef")]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
}

impl Activation {
    pub fn validate(&self) -> Result<()> {
        self.validate_cv_params("activation")?;
        Ok(())
    }
}

impl IsElement for Activation {
    fn validate(&self) -> Result<()> {
        for referenceable_param_group_ref in &self.referenceable_param_group_ref {
            referenceable_param_group_ref.validate()?;
        }
        self.validate_cv_params("activation")?;
        Ok(())
    }
}

has_cv_params! {
    Activation,
    cv_params,
    [],
    [
        "MS:1000044", // dissociation method
    ],
    [],
    [
        "MS:1000510", // precursor activation attribute
    ]
}
