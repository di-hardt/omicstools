use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    activation::Activation, is_element::IsElement, isolation_window::IsolationWindow,
    selected_ion_list::SelectedIonList,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Precursor {
    #[serde(rename = "@spectrumRef")]
    pub spectrum_ref: String,
    #[serde(rename = "isolationWindow")]
    pub isolation_window: IsolationWindow,
    #[serde(rename = "selectedIonList")]
    pub selected_ion_list: SelectedIonList,
    #[serde(rename = "activation")]
    pub activation: Activation,
}

impl IsElement for Precursor {
    fn validate(&self) -> Result<()> {
        self.isolation_window.validate()?;
        self.selected_ion_list.validate()?;
        self.activation.validate()?;
        Ok(())
    }
}
