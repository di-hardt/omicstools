// 3rd party imports
use serde::{Deserialize, Serialize};

use super::{
    component_list::ComponentList, referenceable_param_group_ref::ReferenceableParamGroupRef,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentConfiguration {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "referenceableParamGroupRef")]
    pub referenceable_param_group_ref: ReferenceableParamGroupRef,
    #[serde(rename = "componentList")]
    pub component_list: ComponentList,
}
