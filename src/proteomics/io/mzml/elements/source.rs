use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement};
use crate::has_cv_params;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "@order")]
    pub order: usize,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for Source {
    fn validate(&self) -> Result<()> {
        self.validate_cv_params("source")?;
        Ok(())
    }
}

has_cv_params! {
    Source,
    cv_params,
    [
        "MS:1000008", // ionization type
    ],
    [],
    [
        "MS:1000007", // inlet type
    ],
    [
        "MS:1000482", // source attribute
    ]
}
