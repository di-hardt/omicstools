// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::{cv_param::CvParam, user_param::UserParam};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingMethod {
    #[serde(rename = "@order")]
    pub order: usize,
    #[serde(rename = "@softwareRef")]
    pub software_ref: String,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
}
