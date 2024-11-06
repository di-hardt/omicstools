use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{binary_data_array_list::BinaryDataArrayList, cv_param::CvParam};

#[derive(Debug, Serialize, Deserialize)]
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

impl Chromatogram {
    pub fn validate(&self) -> Result<()> {
        for binary_data_array_list in &self.binary_data_array_list {
            binary_data_array_list.validate()?;
        }
        Ok(())
    }
}
