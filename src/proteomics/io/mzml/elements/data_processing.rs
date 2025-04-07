use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{is_element::IsElement, processing_method::ProcessingMethod};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataProcessing {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(default, rename = "processingMethod")]
    pub processing_methods: Vec<ProcessingMethod>,
}

impl IsElement for DataProcessing {
    fn validate(&self) -> Result<()> {
        for processing_method in &self.processing_methods {
            processing_method.validate()?;
        }
        Ok(())
    }
}
