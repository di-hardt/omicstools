use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::is_element::IsElement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReferenceableParamGroupRef {
    #[serde(rename = "@ref")]
    pub r#ref: String,
}

impl IsElement for ReferenceableParamGroupRef {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
