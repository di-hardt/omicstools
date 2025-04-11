// 3rd party imports
use serde::{Deserialize, Serialize};

use crate::has_cv_params;

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
        self.validate_cv_params("referenceableParamGroup")?;
        Ok(())
    }
}

has_cv_params! {
    ReferenceableParamGroup,
    cv_params,
    [],
    [],
    [],
    []
}
