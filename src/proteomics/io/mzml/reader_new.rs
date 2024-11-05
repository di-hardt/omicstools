use std::io::BufRead;

// 3rd party imports
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

// Local imports
use super::elements::indexed_mz_ml::IndexedMzML;
use super::elements::mz_ml::MzML as MzMLElement;
use super::elements::run::Run;

/// MzML returned by the reader, can be either MzML or IndexedMzML
/// depending on the input file.
///
#[derive(Debug, Serialize, Deserialize)]
pub enum MzML {
    #[serde(rename = "mzML")]
    MzML(MzMLElement<Run>),
    #[serde(rename = "indexedmzML")]
    IndexedMzML(IndexedMzML<Run>),
}

/// Reader for mzML and indexed mzML files. Read the content into the memory.
///
pub struct ReaderNew;

impl ReaderNew {
    /// Read the content of the mzML file and return the MzML struct.
    ///
    pub fn read(mzml_file: &mut dyn BufRead) -> Result<MzML> {
        Ok(quick_xml::de::from_reader::<_, MzML>(mzml_file).context("Failed to parse mzML")?)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::{fs::File, io::BufReader, path::PathBuf};

    #[test]
    fn test_reader_new() {
        // let mzml_path = PathBuf::from("test_files/spectra_small.mzML");
        let mzml_path = PathBuf::from("/Users/winkelhardtdi/Documents/datasets/PXD028735/LFQ_Orbitrap_DDA_Condition_A_Sample_Alpha_01.mzML");
        let mut mzml_file = BufReader::with_capacity(2097152, File::open(mzml_path).unwrap());
        let mzml = ReaderNew::read(&mut mzml_file).unwrap();
        match mzml {
            MzML::MzML(mz_ml) => {
                println!("MzML");
                println!("{:?}", mz_ml.data_processing_list);
            }
            MzML::IndexedMzML(indexed_mz_ml) => {
                println!("IndexedMzML");
                println!("{:?}", indexed_mz_ml.mz_ml.data_processing_list);
            }
        }
    }
}
