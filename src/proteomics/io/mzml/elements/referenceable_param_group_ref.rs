// 3rd party imports
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceableParamGroupRef {
    #[serde(rename = "@ref")]
    pub r#ref: String,
}
