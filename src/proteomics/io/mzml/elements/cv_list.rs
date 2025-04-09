use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{cv::Cv, is_element::IsElement};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CvList {
    #[serde(rename = "@count")]
    pub count: isize,
    #[serde(rename = "cv")]
    pub cv: Vec<Cv>,
}

impl IsElement for CvList {
    fn validate(&self) -> Result<()> {
        for cv in &self.cv {
            cv.validate()?;
        }
        Ok(())
    }
}
