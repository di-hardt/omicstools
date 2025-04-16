use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement};
use crate::has_cv_params;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Software {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for Software {
    fn validate(&self) -> Result<()> {
        self.validate_cv_params("software")?;
        Ok(())
    }
}

has_cv_params! {
    Software,
    cv_params,
    [
        "MS:1000531", // software
    ],
    [],
    [],
    []
}
