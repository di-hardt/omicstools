// 3rd party imports
use serde::{Deserialize, Serialize};

use super::data_processing::DataProcessing;

/// Generic approach to create a list of elements. E.g DataProcessingList, InstrumentConfigurationList, etc.
/// TODO: How to pass validations
macro_rules! mz_ml_element_list {
    ($name:ident, $element:ty, $tag:literal, $min:literal, $($max:literal)) => {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct $name {
            #[serde(rename = "@count")]
            pub count: usize,
            #[serde(default, rename = $tag, validate)]
            pub elements: Vec<$element>,
        }
    };
}

mz_ml_element_list!(DataProcessingList, DataProcessing, "dataProcessing");
