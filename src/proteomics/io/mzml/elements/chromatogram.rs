use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::has_cv_params;

use super::{
    binary_data_array_list::BinaryDataArrayList, cv_param::CvParam, is_element::IsElement,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chromatogram {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@defaultArrayLength")]
    pub default_array_length: String,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(rename = "binaryDataArrayList")]
    pub binary_data_array_list: Vec<BinaryDataArrayList>,
}

impl IsElement for Chromatogram {
    fn validate(&self) -> Result<()> {
        for binary_data_array_list in &self.binary_data_array_list {
            binary_data_array_list.validate()?;
        }
        self.validate_cv_params("chromatogram")?;
        Ok(())
    }
}

has_cv_params! {
    Chromatogram,
    cv_params,
    [
        "MS:1000626", // chromatogram type
    ],
    [],
    [],
    [
        "MS:1000808", // chromatogram attribute
    ]
}
