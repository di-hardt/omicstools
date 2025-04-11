use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv_param::CvParam, is_element::IsElement};
use crate::has_cv_params;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileContent {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
}

impl IsElement for FileContent {
    fn validate(&self) -> Result<()> {
        self.validate_cv_params("fileContent")?;
        Ok(())
    }
}

has_cv_params! {
    FileContent,
    cv_params,
    [
    ],
    [
        "MS:1000524", // data file content
    ],
    [
        "MS:1000252", // spectrum representation
    ],
    [
    ]
}
