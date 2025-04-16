use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{binary_data_array::BinaryDataArray, is_element::IsElement, is_list::IsList};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BinaryDataArrayList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "binaryDataArray")]
    pub binary_data_arrays: Vec<BinaryDataArray>,
}

impl BinaryDataArrayList {
    pub fn get_binary_data_array(&self, accession: &str) -> Result<&BinaryDataArray> {
        for binary_data_array in &self.binary_data_arrays {
            if binary_data_array
                .cv_params
                .iter()
                .any(|cv_param| cv_param.accession == accession)
            {
                return Ok(binary_data_array);
            }
        }
        bail!("No binaryDataArray found with accession: {}", accession);
    }

    pub fn get_mz_array(&self) -> Result<&BinaryDataArray> {
        self.get_binary_data_array("MS:1000514")
    }

    pub fn get_intensity_array(&self) -> Result<&BinaryDataArray> {
        self.get_binary_data_array("MS:1000515")
    }
}

impl IsElement for BinaryDataArrayList {
    fn validate(&self) -> Result<()> {
        if self.count < 2 {
            bail!("The count attribute ({}) must be at least 2", self.count);
        }
        if self.count != self.binary_data_arrays.len() {
            bail!(
                "The count attribute ({}) does not match the number of binaryDataArray elements ({})",
                self.count,
                self.binary_data_arrays.len()
            );
        }
        for binary_data_array in &self.binary_data_arrays {
            binary_data_array.validate()?;
        }
        Ok(())
    }
}

impl IsList<'_, BinaryDataArray> for BinaryDataArrayList {
    fn iter(&self) -> std::slice::Iter<'_, BinaryDataArray> {
        self.binary_data_arrays.iter()
    }
}
