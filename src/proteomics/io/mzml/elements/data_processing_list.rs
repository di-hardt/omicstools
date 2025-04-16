use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    data_processing::DataProcessing,
    is_element::IsElement,
    is_list::IsList,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataProcessingList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "dataProcessing")]
    pub data_processings: Vec<DataProcessing>,
}

impl IsElement for DataProcessingList {
    fn validate(&self) -> Result<()> {
        for data_processing in &self.data_processings {
            data_processing.validate()?;
        }
        Ok(())
    }
}

impl IsList<'_, DataProcessing> for DataProcessingList {
    fn iter(&self) -> std::slice::Iter<'_, DataProcessing> {
        self.data_processings.iter()
    }
}
