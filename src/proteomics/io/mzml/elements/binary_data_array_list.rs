use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{binary_data_array::BinaryDataArray, is_element::IsElement};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BinaryDataArrayList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "binaryDataArray")]
    pub binary_data_arrays: Vec<BinaryDataArray>,
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
