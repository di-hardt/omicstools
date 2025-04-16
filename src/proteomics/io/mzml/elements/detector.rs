use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement};
use crate::has_cv_params;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Detector {
    #[serde(rename = "@order")]
    pub order: usize,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for Detector {
    fn validate(&self) -> Result<()> {
        self.validate_cv_params("detector")?;
        Ok(())
    }
}

has_cv_params! {
    Detector,
    cv_params,
    [
        "MS:1000026", // detector type
    ],
    [],
    [],
    [
        "MS:1000481", // detector attribute
        "MS:1000027", // detector acquisition mode
    ]
}
