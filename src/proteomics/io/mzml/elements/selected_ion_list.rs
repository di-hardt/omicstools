use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{is_element::IsElement, is_list::IsList, selected_ion::SelectedIon};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedIonList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "selectedIon")]
    pub selected_ions: Vec<SelectedIon>,
}

impl IsElement for SelectedIonList {
    fn validate(&self) -> Result<()> {
        if self.count != self.selected_ions.len() {
            bail!("SelectedIonList count does not match the number of selectedIon elements");
        }
        for selected_ion in &self.selected_ions {
            selected_ion.validate()?;
        }
        Ok(())
    }
}

impl IsList<'_, SelectedIon> for SelectedIonList {
    fn iter(&self) -> std::slice::Iter<'_, SelectedIon> {
        self.selected_ions.iter()
    }
}
