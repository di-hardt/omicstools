// 3rd party imports
use serde::{Deserialize, Serialize};

use super::{
    is_element::IsElement,
    is_list::IsList,
    referenceable_param_group::ReferenceableParamGroup,
};

// Local imports

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl IsList<'_, ReferenceableParamGroup> for ReferenceableParamGroupList {
    fn iter(&self) -> std::slice::Iter<'_, ReferenceableParamGroup> {
        self.referenceable_param_groups.iter()
    }
}
