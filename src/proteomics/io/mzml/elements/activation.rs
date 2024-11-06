use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::cv_param::CvParam;
use crate::build_cv_params_validator;

#[derive(Debug, Serialize, Deserialize)]

pub struct Activation {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
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
