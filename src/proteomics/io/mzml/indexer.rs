use std::collections::HashMap;
use std::io::prelude::*;

use anyhow::Result;

use crate::proteomics::io::mzml::index::Index;

/// Start of ID attribute
const ID_START: &[u8] = b"id=\"";

/// Attribute end tag
const ATTRIBUTE_END: &[u8] = b"\"";

/// Default chunk size to read from file (1MB)
const DEFAULT_BUFFER_SIZE: usize = 1024 * 1000;

/// Creates an index of the given mzML file by
/// finding the length of the general information (everything before the spectrumList),
/// each spectrum's start and end offset adn the default data processing reference and indention
/// (for extraction of a single spectrum into a separate valid mzML file).
/// TODO: Index chromatograms
///
pub struct Indexer<'a, F>
where
    F: BufRead + Seek,
{
    mzml_file: &'a mut F,
    buffer_size: usize,
}

impl<'a, F> Indexer<'a, F>
where
    F: BufRead + Seek,
{
    pub fn new(reader: &'a mut F, buffer_size: Option<usize>) -> Self {
        Self {
            mzml_file: reader,
            buffer_size: buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE),
        }
    }

    /// Returns the spectrum offsets, beginning with '<spectrum ' and ending with </spectrum>.
    ///
    fn create_idx(&mut self) -> Result<Index> {
        self.mzml_file.seek(std::io::SeekFrom::Start(0))?;
        let mut buffer = Vec::with_capacity(self.buffer_size);
        let mut reader = quick_xml::Reader::from_reader(&mut self.mzml_file);
        let mut spectrum_offsets: HashMap<String, usize> = HashMap::new();
        let mut chromatogram_offsets: HashMap<String, usize> = HashMap::new();
        loop {
            match reader.read_event_into(&mut buffer) {
                Ok(quick_xml::events::Event::Start(ref e)) => match e.local_name().as_ref() {
                    b"spectrum" => {
                        spectrum_offsets.insert(
                            get_id_attributes(e)?,
                            reader.buffer_position() as usize - e.len() - 2, // position reports last byte of start tag `>`, mzML wants the first byte `<`
                        );
                    }
                    b"chromatogram" => {
                        chromatogram_offsets.insert(
                            get_id_attributes(e)?,
                            reader.buffer_position() as usize - e.len() - 2, // position reports last byte of start tag `>`, mzML wants the first byte `<`
                        );
                    }
                    _ => (),
                },
                Ok(quick_xml::events::Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
        }
        Ok(Index::new(spectrum_offsets, chromatogram_offsets))
    }

    /// Creates a file index for the given mzML file.
    ///
    /// # Arguments
    /// * `reader`- Open reader.
    /// * `buffer_size` - Size of the chunks to read from the file.
    ///
    pub fn create_index(reader: &'a mut F, buffer_size: Option<usize>) -> Result<Index> {
        Self::new(reader, buffer_size).create_idx()
    }
}

pub fn get_id_attributes(id: &[u8]) -> Result<String> {
    let id_start = id
        .windows(ID_START.len())
        .position(|x| x == ID_START)
        .ok_or_else(|| {
            anyhow::anyhow!("No id attribute found. `{}`", String::from_utf8_lossy(id))
        })?
        + ID_START.len();
    let id_end = id[id_start..]
        .windows(ATTRIBUTE_END.len())
        .position(|x| x == ATTRIBUTE_END)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "ID has no end, broken tag? `{}`",
                String::from_utf8_lossy(id)
            )
        })?
        + id_start;
    Ok(String::from_utf8_lossy(&id[id_start..id_end]).to_string())
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufReader, path::Path};

    use super::*;

    const EXPECTED_SPECTRA: [(&str, (usize, usize)); 11] = [
        (
            "controllerType=0 controllerNumber=1 scan=4051",
            (59212, 67042),
        ),
        (
            "controllerType=0 controllerNumber=1 scan=4052",
            (67051, 75575),
        ),
        (
            "controllerType=0 controllerNumber=1 scan=4053",
            (75584, 86661),
        ),
        (
            "controllerType=0 controllerNumber=1 scan=4050",
            (50455, 59203),
        ),
        (
            "controllerType=0 controllerNumber=1 scan=3865",
            (22071, 28215),
        ),
        (
            "controllerType=0 controllerNumber=1 scan=4045",
            (28224, 35295),
        ),
        (
            "controllerType=0 controllerNumber=1 scan=3224",
            (15920, 22062),
        ),
        (
            "controllerType=0 controllerNumber=1 scan=2814",
            (4072, 9932),
        ),
        (
            "controllerType=0 controllerNumber=1 scan=4046",
            (35304, 41934),
        ),
        (
            "controllerType=0 controllerNumber=1 scan=4049",
            (41943, 50446),
        ),
        (
            "controllerType=0 controllerNumber=1 scan=2958",
            (9941, 15911),
        ),
    ];

    const EXPECTED_CROMATOGRAMS: [(&str, (usize, usize)); 1] = [("TIC", (86784, 622457))];

    #[test]
    fn test_id_extraction() {
        let id = b"spectrum index=\"0\" id=\"controllerType=0 controllerNumber=1 scan=2814\" defaultArrayLength=\"29\"";
        let id_str = get_id_attributes(id).unwrap();
        assert_eq!(
            id_str,
            "controllerType=0 controllerNumber=1 scan=2814".to_string()
        );
    }

    #[test]
    fn test_index_creation() {
        let file_path = Path::new("./test_files/spectra_small.mzML");
        let mut buf_reader = BufReader::new(File::open(file_path).unwrap());

        let now = std::time::Instant::now();
        let index = Indexer::create_index(&mut buf_reader, None).unwrap();
        println!("Index creation took: {:?}", now.elapsed());

        assert_eq!(index.get_spectra().len(), EXPECTED_SPECTRA.len());

        for (spec_id, expected_start_end) in EXPECTED_SPECTRA {
            assert!(index.get_spectra().contains_key(spec_id));
            if let Some(start_end) = index.get_spectra().get(spec_id) {
                assert_eq!(expected_start_end.0, *start_end);
            }
        }

        assert_eq!(index.get_chromatograms().len(), EXPECTED_CROMATOGRAMS.len());
        for (chrom_id, expected_start_end) in EXPECTED_CROMATOGRAMS {
            assert!(index.get_chromatograms().contains_key(chrom_id));
            if let Some(start_end) = index.get_chromatograms().get(chrom_id) {
                assert_eq!(expected_start_end.0, *start_end);
            }
        }
    }

    #[test]
    fn test_testy() {
        let file_path = Path::new("/Users/winkelhardtdi/Documents/datasets/PXD028735/LFQ_Orbitrap_DDA_Condition_A_Sample_Alpha_01.mzML");
        let mut buf_reader = BufReader::new(File::open(file_path).unwrap());
        let now = std::time::Instant::now();
        let _ = Indexer::create_index(&mut buf_reader, None).unwrap();
        println!("Index creation took: {:?}", now.elapsed());
    }
}
