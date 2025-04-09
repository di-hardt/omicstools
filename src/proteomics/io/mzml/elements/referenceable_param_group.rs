// 3rd party imports
use serde::{Deserialize, Serialize};

use crate::build_cv_params_validator;

// Local imports
use super::{cv_param::CvParam, is_element::IsElement};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReferenceableParamGroup {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for ReferenceableParamGroup {
    fn validate(&self) -> anyhow::Result<()> {
        for cv_param in &self.cv_params {
            cv_param.validate()?;
        }
        self.validate_cv_params(&self.cv_params, "referenceableParamGroup")?;
        Ok(())
    }
}

build_cv_params_validator! {
    ReferenceableParamGroup,
    [],
    [],
    [],
    []
}
