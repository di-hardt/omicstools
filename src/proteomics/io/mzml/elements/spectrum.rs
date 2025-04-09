use serde::{Deserialize, Serialize};

use anyhow::Result;

use crate::build_cv_params_validator;

use super::{
    binary_data_array_list::BinaryDataArrayList, cv_param::CvParam, is_element::IsElement,
    precursor_list::PrecursorList, scan_list::ScanList,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Spectrum {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@defaultArrayLength")]
    pub default_array_length: String,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(rename = "scanList")]
    pub scan_list: ScanList,
    #[serde(rename = "precursorList", skip_serializing_if = "Option::is_none")]
    pub precursor_list: Option<PrecursorList>,
    #[serde(rename = "binaryDataArrayList")]
    pub binary_data_array_list: BinaryDataArrayList,
}

impl IsElement for Spectrum {
    fn validate(&self) -> Result<()> {
        self.scan_list.validate()?;
        if let Some(ref precursor_list) = self.precursor_list {
            precursor_list.validate()?;
        }
        self.binary_data_array_list.validate()?;
        Ok(())
    }
}

build_cv_params_validator! {
    Spectrum,
    [
        "MS:1000559", // spectrum type
        "MS:1000525", // spectrum representation
    ],
    [],
    [
        "MS:1000465", // scan polarity
    ],
    [
        "MS:1000499", // spectrum attribute
    ]
}
