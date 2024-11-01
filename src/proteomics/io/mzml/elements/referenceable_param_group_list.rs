// 3rd party imports
use serde::{Deserialize, Serialize};

use super::referenceable_param_group::ReferenceableParamGroup;

// Local imports

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceableParamGroupList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "referenceableParamGroup")]
    pub referenceable_param_groups: Vec<ReferenceableParamGroup>,
}
