use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    cv_param::CvParam, is_element::IsElement, scan_window_list::ScanWindowList,
    user_param::UserParam,
};
use crate::has_cv_params;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scan {
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
    #[serde(rename = "scanWindowList")]
    pub scan_window_list: ScanWindowList,
}

impl IsElement for Scan {
    fn validate(&self) -> Result<()> {
        self.validate_cv_params("scan")?;
        for user_param in &self.user_params {
            user_param.validate()?;
        }
        self.scan_window_list.validate()?;
        Ok(())
    }
}

has_cv_params! {
    Scan,
    cv_params,
    [],
    [],
    [
        "MS:1000018", // scan direction
        "MS:1000019", // scan law
    ],
    [
        "MS:1000503", // scan attribute
    ]
}
