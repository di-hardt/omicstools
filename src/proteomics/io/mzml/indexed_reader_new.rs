use std::io::{BufRead, Seek, SeekFrom};

// 3rd party imports
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

// Local imports
use super::elements::{chromatogram::Chromatogram, indexed_mz_ml::IndexedMzML, spectrum::Spectrum};

/// Implementation of the MzML element <run> without spectrum and chromatogram data.
/// This is useful for indexing the MzML file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Run {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@defaultInstrumentConfigurationRef")]
    pub default_instrument_configuration_ref: String,
    #[serde(rename = "@startTimeStamp")]
    pub start_time_stamp: String,
    #[serde(rename = "@defaultSourceFileRef")]
    pub default_source_file_ref: String,
}

pub struct IndexedMzMLReader<F>
where
    F: BufRead + Seek,
{
    pub indexed_mz_ml: IndexedMzML<Run>,
    reader: F,
}

impl<F> IndexedMzMLReader<F>
where
    F: BufRead + Seek,
{
    pub fn read(mut indexed_mzml_file: F) -> Result<Self> {
        let indexed_mz_ml =
            quick_xml::de::from_reader::<_, IndexedMzML<Run>>(&mut indexed_mzml_file)
                .context("Failed to parse indexed mzML")?;
        indexed_mzml_file.seek(std::io::SeekFrom::Start(0))?;
        Ok(Self {
            indexed_mz_ml,
            reader: indexed_mzml_file,
        })
    }

    pub fn get_spectrum(&mut self, spectrum_id: &str) -> Result<Spectrum> {
        let spectrum_index = self
            .indexed_mz_ml
            .index_list
            .indexes
            .iter()
            .find(|i| i.name == "spectrum");

        if spectrum_index.is_none() {
            return Err(anyhow::anyhow!("Spectrum index not found"));
        }

        let spectrum_index = spectrum_index.unwrap();

        for i in spectrum_index.offsets.iter() {
            if i.id_ref == spectrum_id {
                self.reader.seek(SeekFrom::Start(i.value as u64))?;
                return quick_xml::de::from_reader::<_, Spectrum>(&mut self.reader)
                    .context("Failed to parse spectrum");
            }
        }
        Err(anyhow::anyhow!("Spectrum not found"))
    }

    pub fn get_chromatogram(&mut self, chromatogram_id: &str) -> Result<Chromatogram> {
        let chromatogram_index = self
            .indexed_mz_ml
            .index_list
            .indexes
            .iter()
            .find(|i| i.name == "chromatogram");

        if chromatogram_index.is_none() {
            return Err(anyhow::anyhow!("Chromatogram index not found"));
        }

        let spectrum_index = chromatogram_index.unwrap();

        for i in spectrum_index.offsets.iter() {
            if i.id_ref == chromatogram_id {
                self.reader.seek(SeekFrom::Start(i.value as u64))?;
                return quick_xml::de::from_reader::<_, Chromatogram>(&mut self.reader)
                    .context("Failed to parse chromatogram");
            }
        }
        Err(anyhow::anyhow!("Chromatogram not found"))
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::{fs::File, io::BufReader, path::PathBuf};

    #[test]
    fn test_reader_new() {
        let mzml_path = PathBuf::from("test_files/spectra_small.mzML");
        // let mzml_path = PathBuf::from("/Users/winkelhardtdi/Documents/datasets/PXD028735/LFQ_Orbitrap_DDA_Condition_A_Sample_Alpha_01.mzML");
        let mut mzml_file = BufReader::with_capacity(2097152, File::open(mzml_path).unwrap());
        let mut reader = IndexedMzMLReader::read(&mut mzml_file).unwrap();
        let spectrum = reader
            .get_spectrum("controllerType=0 controllerNumber=1 scan=4045")
            .unwrap();
        println!("{:?}", spectrum);

        let chromatogram = reader.get_chromatogram("TIC").unwrap();
        println!("{:?}", chromatogram);
    }
}
