use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::has_cv_params;

use super::{cv_param::CvParam, is_element::IsElement, scan::Scan};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "scan")]
    pub scans: Vec<Scan>,
}

impl IsElement for ScanList {
    fn validate(&self) -> Result<()> {
        if self.count != self.scans.len() {
            bail!(
                "The count attribute ({}) does not match the number of scan elements ({})",
                self.count,
                self.scans.len()
            );
        }
        self.validate_cv_params("scanList")?;

        for scan in &self.scans {
            scan.validate()?;
        }
        Ok(())
    }
}

has_cv_params! {
    ScanList,
    cv_params,
    [
        "MS:1000570", // spectra combination
    ],
    [],
    [],
    []
}
