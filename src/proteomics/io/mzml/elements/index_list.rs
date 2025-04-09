use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{index::Index, is_element::IsElement, offset::Offset};
use crate::proteomics::io::mzml::index::Index as MzmlIndex;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "index")]
    pub indexes: Vec<Index>,
}

impl<'a> IndexList {
    pub fn get_index_by_name(&'a self, name: &str) -> Option<&'a Index> {
        self.indexes.iter().find(|index| index.name == name)
    }
}

impl IsElement for IndexList {
    fn validate(&self) -> Result<()> {
        for index in &self.indexes {
            index.validate()?;
        }
        Ok(())
    }
}

impl From<MzmlIndex> for IndexList {
    fn from(index: MzmlIndex) -> Self {
        let mut spectrum_offsets = index
            .get_spectra()
            .iter()
            .map(|(id, offset)| Offset {
                id_ref: id.clone(),
                value: *offset,
            })
            .collect::<Vec<_>>();
        spectrum_offsets.sort_by_key(|offset| offset.value);

        let mut chromatogram_offsets = index
            .get_chromatograms()
            .iter()
            .map(|(id, offset)| Offset {
                id_ref: id.clone(),
                value: *offset,
            })
            .collect::<Vec<_>>();
        chromatogram_offsets.sort_by_key(|offset| offset.value);

        let spectrum_index = Index {
            name: "spectrum".to_string(),
            offsets: spectrum_offsets,
        };

        let chromatogram_index = Index {
            name: "chromatogram".to_string(),
            offsets: chromatogram_offsets,
        };

        Self {
            count: 2,
            indexes: vec![spectrum_index, chromatogram_index],
        }
    }
}
