use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement};
use crate::has_cv_params;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedIon {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for SelectedIon {
    fn validate(&self) -> Result<()> {
        self.validate_cv_params("selectedIon")?;
        Ok(())
    }
}

has_cv_params! {
    SelectedIon,
    cv_params,
    [],
    [
        "MS:1000455", // ion selection attribute
    ],
    [],
    []
}
