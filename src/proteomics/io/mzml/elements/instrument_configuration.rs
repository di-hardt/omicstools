use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    component_list::ComponentList, is_element::IsElement,
    referenceable_param_group_ref::ReferenceableParamGroupRef,
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

impl IsElement for InstrumentConfiguration {
    fn validate(&self) -> Result<()> {
        self.referenceable_param_group_ref.validate()?;
        self.component_list.validate()?;
        Ok(())
    }
}
