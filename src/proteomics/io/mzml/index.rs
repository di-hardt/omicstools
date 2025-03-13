// std imports
use std::collections::HashMap;

// 3rd party crates
use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::elements::index_list::IndexList;

/// Index for a mzML file.
/// Stores length of the the general information (everything before the spectrumList) ends,
/// each spectrum's start and end offset and the default data processing reference and indention
/// (for extraction of a single spectrum into a separate valid mzML file).
/// TODO: Index chromatograms
///
#[derive(Serialize, Deserialize)]
pub struct Index {
    spectra: HashMap<String, usize>,
    chromatograms: HashMap<String, usize>,
}

impl Index {
    pub fn new(spectra: HashMap<String, usize>, chromatograms: HashMap<String, usize>) -> Self {
        Self {
            spectra,
            chromatograms,
        }
    }

    pub fn get_spectra(&self) -> &HashMap<String, usize> {
        &self.spectra
    }

    pub fn get_chromatograms(&self) -> &HashMap<String, usize> {
        &self.chromatograms
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
}

impl From<&IndexList> for Index {
    fn from(index_list: &IndexList) -> Self {
        let spectrum_index = match index_list.get_index_by_name("spectrum") {
            Some(index) => index
                .offsets
                .iter()
                .map(|offset| (offset.id_ref.to_string(), offset.value))
                .collect(),
            None => HashMap::with_capacity(0),
        };
        let chromatogram_index = match index_list.get_index_by_name("chromatogram") {
            Some(index) => index
                .offsets
                .iter()
                .map(|offset| (offset.id_ref.to_string(), offset.value))
                .collect(),
            None => HashMap::with_capacity(0),
        };
        Index::new(spectrum_index, chromatogram_index)
    }
}

#[cfg(test)]
mod test {
    // std
    use std::{io::BufReader, path::Path};

    // internal
    use super::*;
    use crate::proteomics::io::mzml::indexer::Indexer;

    #[test]
    fn test_serialization_deserialization() {
        let file_path = Path::new("./test_files/spectra_small.mzML");
        let mut reader = BufReader::new(std::fs::File::open(file_path).unwrap());

        let index = Indexer::create_index(&mut reader, None).unwrap();

        let serialized = index.to_json().unwrap();
        let deserialized_index: Index = Index::from_json(&serialized).unwrap();

        for (spec_id, expected_start_end_offset) in index.get_spectra().iter() {
            assert!(deserialized_index.get_spectra().contains_key(spec_id));
            if let Some(start_end_offset) = deserialized_index.get_spectra().get(spec_id) {
                assert_eq!(expected_start_end_offset, start_end_offset);
                assert_eq!(expected_start_end_offset, start_end_offset)
            }
        }
        // TODO: add chromatogram assertion when they get added
    }
}
