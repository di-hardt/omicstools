// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::{
    activation::Activation, isolation_window::IsolationWindow, selected_ion_list::SelectedIonList,
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
    pub activations: Activation,
}
