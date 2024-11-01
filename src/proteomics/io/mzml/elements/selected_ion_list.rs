// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::selected_ion::SelectedIon;

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectedIonList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "selectedIon")]
    pub selected_ions: Vec<SelectedIon>,
}
