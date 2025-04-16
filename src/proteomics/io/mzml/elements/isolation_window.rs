use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::has_cv_params;

// Local imports
use super::{cv_param::CvParam, is_element::IsElement};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IsolationWindow {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for IsolationWindow {
    fn validate(&self) -> Result<()> {
        self.validate_cv_params("activation")?;
        Ok(())
    }
}

has_cv_params! {
    IsolationWindow,
    cv_params,
    [
    ],
    [
        "MS:1000792", // isolation window attribute
    ],
    [],
    [
    ]
}
