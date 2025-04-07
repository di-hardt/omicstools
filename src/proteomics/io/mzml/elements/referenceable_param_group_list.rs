// 3rd party imports
use serde::{Deserialize, Serialize};

use super::{is_element::IsElement, referenceable_param_group::ReferenceableParamGroup};

// Local imports

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceableParamGroupList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "referenceableParamGroup")]
    pub referenceable_param_groups: Vec<ReferenceableParamGroup>,
}

impl IsElement for ReferenceableParamGroupList {
    fn validate(&self) -> anyhow::Result<()> {
        for referenceable_param_group in &self.referenceable_param_groups {
            referenceable_param_group.validate()?;
        }
        Ok(())
    }
}
