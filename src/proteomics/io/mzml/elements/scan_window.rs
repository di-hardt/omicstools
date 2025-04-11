use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement};
use crate::has_cv_params;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanWindow {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for ScanWindow {
    fn validate(&self) -> Result<()> {
        self.validate_cv_params("ScanWindow")?;
        Ok(())
    }
}

has_cv_params! {
    ScanWindow,
    cv_params,
    [
        "MS:1000500", // scan window upper limit
        "MS:1000501", // scan window lower limit
    ],
    [
        "MS:1000549", // selection window attribute
    ],
    [],
    []
}
