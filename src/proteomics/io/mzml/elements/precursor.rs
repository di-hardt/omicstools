use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    activation::Activation, is_element::IsElement, isolation_window::IsolationWindow,
    selected_ion_list::SelectedIonList,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Precursor {
    #[serde(rename = "@spectrumRef")]
    pub spectrum_ref: String,
    #[serde(rename = "isolationWindow")]
    pub isolation_window: Option<IsolationWindow>,
    #[serde(rename = "selectedIonList")]
    pub selected_ion_list: Option<SelectedIonList>,
    #[serde(rename = "activation")]
    pub activation: Activation,
}

impl IsElement for Precursor {
    fn validate(&self) -> Result<()> {
        if let Some(isolation_window) = self.isolation_window.as_ref() {
            isolation_window.validate()?;
        }
        if let Some(selected_ion_list) = self.selected_ion_list.as_ref() {
            selected_ion_list.validate()?;
        }
        self.activation.validate()?;
        Ok(())
    }
}
