use core::fmt::Write;
use std::io::{BufRead, Seek, SeekFrom};
use std::ops::Deref;
use std::vec;

use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use sha1::{Digest, Sha1};

use super::elements::chromatogram::Chromatogram;
use super::elements::indexed_mz_ml::IndexedMzML;
use super::elements::is_element::IsElement;
use super::elements::mz_ml::MzML;
use super::elements::run::{IndexedRun, Run};
use super::elements::spectrum::Spectrum;
use super::index::Index;
use super::indexer::Indexer;

/// Default buffer size
///
const DEFAULT_BUFFER_SIZE: usize = 1024; // 1kb

/// IndexedMzML tag
///
const INDEXED_MZML_START_TAG: &[u8; 12] = b"<indexedmzML";

/// MzML staret tag
///
const MZML_START_TAG: &[u8; 5] = b"<mzML";

/// Opening spectrum list tag + two spaces
///
const OPENING_SPECTRUM_LIST_TAG: &[u8; 13] = b"<spectrumList";

/// Closing run tag in reverse
///
const CLOSING_RUN_TAG_REV: &[u8; 6] = b">nur/<";

/// Opening filechecksum tag
///
const OPENING_FILECHECKSUM_TAG: &[u8; 14] = b"<fileChecksum>";

/// Closing filechecksum tag
///
const CLOSING_FILECHECKSUM_TAG: &[u8; 15] = b"</fileChecksum>";

/// Opening indexList tag
///
const OPENING_INDEX_LIST_TAG: &[u8; 10] = b"<indexList";

/// Opening indexListOffset tag
///
const OPENING_INDEX_LIST_OFFSET_TAG: &[u8; 17] = b"<indexListOffset>";

/// Closing indexListOffset tag
///
const CLOSING_INDEX_LIST_OFFSET_TAG: &[u8; 18] = b"</indexListOffset>";

#[derive(Clone)]
pub enum MzMlElement {
    MzML(MzML<IndexedRun>),
    IndexedMzML(IndexedMzML<IndexedRun>),
}

/// Open MzML with random access to spectra and chromatograms.
///
pub struct File<'a, F>
where
    F: BufRead + Seek,
{
    /// Internal reader
    reader: &'a mut F,
    /// Either MzML or IndexedMzML as dereferenced
    mzml_element: MzMlElement,
    /// Spectrum index
    index: Index,
}

impl<'a, F> File<'a, F>
where
    F: BufRead + Seek,
{
    /// Returns a spectrum by ID
    ///
    pub fn get_spectrum(&mut self, spectrum_id: &str) -> Result<Spectrum> {
        let offset = match self.index.get_spectra().get(spectrum_id) {
            Some(offset) => offset,
            None => return Err(anyhow::anyhow!("Spectrum not found")),
        };

        self.reader.seek(SeekFrom::Start(*offset as u64))?;
        quick_xml::de::from_reader::<_, Spectrum>(&mut self.reader)
            .context("Failed to parse spectrum")
    }

    /// Returns a chromatogram by ID
    ///
    pub fn get_chromatogram(&mut self, chromatogram_id: &str) -> Result<Chromatogram> {
        let offset = match self.index.get_chromatograms().get(chromatogram_id) {
            Some(offset) => offset,
            None => return Err(anyhow::anyhow!("Chromatogram not found")),
        };

        self.reader.seek(SeekFrom::Start(*offset as u64))?;
        quick_xml::de::from_reader::<_, Chromatogram>(&mut self.reader)
            .context("Failed to parse chromatogram")
    }

    /// Returns a valid mzML with the given spectrum.
    ///
    pub fn extract_spectrum(
        &'a mut self,
        spectrum_id: &str,
        include_parents: bool,
    ) -> Result<String> {
        let mut spectra: Vec<Spectrum> = Vec::with_capacity(2); // expect ms level 2 maybe 3
        let mut next_spec_ids = vec![spectrum_id.to_string()];
        while let Some(spectrum_id) = next_spec_ids.pop() {
            let spectrum = self.get_spectrum(&spectrum_id)?;

            if include_parents {
                if let Some(precursor_list) = &spectrum.precursor_list {
                    for precursor in precursor_list.precursors.iter() {
                        next_spec_ids.push(precursor.spectrum_ref.clone())
                    }
                }
            }
            spectra.push(spectrum);
        }
        spectra.sort_by(|x, y| x.index.cmp(&y.index));
        let mzml = self.mzml_element.clone();

        match mzml {
            MzMlElement::MzML(mzml) => self.extract_spectrum_from_mzml(mzml, spectra),
            MzMlElement::IndexedMzML(indexed_mzml) => {
                self.extract_spectrum_from_indexed_mzml(indexed_mzml, spectra)
            }
        }
    }

    fn extract_spectrum_from_mzml(
        &mut self,
        mzml_element: MzML<IndexedRun>,
        spectra: Vec<Spectrum>,
    ) -> Result<String> {
        let mut mzml_element: MzML<Run> = mzml_element.into();
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
        let mut ser = quick_xml::se::Serializer::with_root(&mut xml, Some("mzML"))?;
        ser.indent(' ', 2);
        mzml_element.run.spectrum_list.count = spectra.len();
        mzml_element.run.spectrum_list.spectra = spectra;
        mzml_element.serialize(ser)?;
        Ok(xml)
    }

    fn extract_spectrum_from_indexed_mzml(
        &mut self,
        indexed_mzml_element: IndexedMzML<IndexedRun>,
        spectra: Vec<Spectrum>,
    ) -> Result<String> {
        // prepare serialization
        let mut indexed_mzml_element: IndexedMzML<Run> = indexed_mzml_element.into();
        indexed_mzml_element.mz_ml.run.spectrum_list.count = spectra.len();
        indexed_mzml_element.mz_ml.run.spectrum_list.spectra = spectra;

        // serialize
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
        let mut ser = quick_xml::se::Serializer::with_root(&mut xml, Some("indexedmzML"))?;
        ser.indent(' ', 2);
        indexed_mzml_element.serialize(ser)?;

        // index the spectra
        let mut cursor = std::io::Cursor::new(&xml);
        let index = Indexer::create_index(&mut cursor, None)?;
        indexed_mzml_element.index_list = index.into();

        // serialize again with new index
        xml.clear();
        xml.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
        let mut ser = quick_xml::se::Serializer::with_root(&mut xml, Some("indexedmzML"))?;
        ser.indent(' ', 2);
        indexed_mzml_element.serialize(ser)?;

        // Set correct indexListOffset
        let index_list_offset = xml
            .find(String::from_utf8_lossy(OPENING_INDEX_LIST_TAG).as_ref())
            .ok_or_else(|| {
                anyhow!(
                    "Failed to find `{}` tag",
                    String::from_utf8_lossy(OPENING_INDEX_LIST_OFFSET_TAG)
                )
            })?;

        let index_list_offset_start_position = xml
            .find(String::from_utf8_lossy(OPENING_INDEX_LIST_OFFSET_TAG).as_ref())
            .ok_or_else(|| {
                anyhow!(
                    "Failed to find `{}` tag",
                    String::from_utf8_lossy(OPENING_INDEX_LIST_OFFSET_TAG)
                )
            })?
            + OPENING_INDEX_LIST_OFFSET_TAG.len();

        let index_list_offset_end_position = xml[index_list_offset_start_position..]
            .find(String::from_utf8_lossy(CLOSING_INDEX_LIST_OFFSET_TAG).as_ref())
            .ok_or_else(|| {
                anyhow!(
                    "Failed to find `{}` tag",
                    String::from_utf8_lossy(CLOSING_INDEX_LIST_OFFSET_TAG)
                )
            })?
            + index_list_offset_start_position;

        xml = format!(
            "{}{}{}",
            &xml[..index_list_offset_start_position],
            index_list_offset,
            &xml[index_list_offset_end_position..]
        );

        // Calculate new filechecksum and replace it in the xml
        let filechecksum_start_position = xml
            .find(String::from_utf8_lossy(OPENING_FILECHECKSUM_TAG).as_ref())
            .ok_or_else(|| {
                anyhow!(
                    "Failed to find `{}` tag",
                    String::from_utf8_lossy(OPENING_FILECHECKSUM_TAG)
                )
            })?
            + OPENING_FILECHECKSUM_TAG.len();

        let filechecksum_end_position = xml[filechecksum_start_position..]
            .find(String::from_utf8_lossy(CLOSING_FILECHECKSUM_TAG).as_ref())
            .ok_or_else(|| {
                anyhow!(
                    "Failed to find `{}` tag",
                    String::from_utf8_lossy(CLOSING_FILECHECKSUM_TAG)
                )
            })?
            + filechecksum_start_position;

        let mut hasher = Sha1::new();
        hasher.update(&xml[..filechecksum_start_position]);

        let hash_result = hasher.finalize();

        // hex conversion
        let mut hex_hash = String::with_capacity(2 * hash_result.len());
        for byte in hash_result {
            write!(hex_hash, "{:02x}", byte)?;
        }

        Ok(format!(
            "{}{}{}",
            &xml[..filechecksum_start_position],
            hex_hash,
            &xml[filechecksum_end_position..]
        ))
    }
}

impl<F> Deref for File<'_, F>
where
    F: BufRead + Seek,
{
    type Target = MzMlElement;

    fn deref(&self) -> &Self::Target {
        &self.mzml_element
    }
}

/// This reader will return and mzML with indexed spectra data,
/// regardless if the provided mzML is indexedmzML or mzML.
///
pub struct Reader<F> {
    _phantom_buf_read: std::marker::PhantomData<F>,
}

impl<F> Reader<F>
where
    F: BufRead + Seek,
{
    /// Read the content of the mzML file and returns a File object.
    /// The object does not read the specturm or chromatogram data immediatly
    /// but creates an index to access the data by ther id when necessary
    /// Works for indexedmzML and mzML.
    ///
    /// # Arguments
    /// * `mzml_file` - A mutable reference to a BufRead and Seek object.
    /// * `create_index` - A boolean flag to create an index for the mzML file.
    /// * `buffer_size` - An optional buffer size for reading the mzML file.
    /// * `reindex` - A boolean flag to reindex an indexed mzML file instead of using the provided index.
    /// * `validate_mzml` - A boolean flag to validate the mzML file.
    ///
    pub fn read_indexed(
        mzml_file: &mut F,
        buffer_size: Option<usize>,
        force_reindex: bool,
        validate_mzml: bool,
    ) -> Result<File<F>> {
        mzml_file.seek(SeekFrom::Start(0))?;

        let mzml_without_data = Reader::get_mzml_without_data(mzml_file, buffer_size)?;

        if mzml_without_data
            .windows(INDEXED_MZML_START_TAG.len())
            .any(|window| window == INDEXED_MZML_START_TAG)
        {
            let indexed_mzml = quick_xml::de::from_reader::<_, IndexedMzML<IndexedRun>>(
                mzml_without_data.as_slice(),
            )
            .context("Failed to parse indexed mzML")?;
            if validate_mzml {
                indexed_mzml.validate()?;
            }

            let index = if force_reindex {
                Indexer::create_index(mzml_file, buffer_size)?
            } else {
                Index::from(&indexed_mzml.index_list)
            };

            return Ok(File {
                index,
                reader: mzml_file,
                mzml_element: MzMlElement::IndexedMzML(indexed_mzml),
            });
        }

        if mzml_without_data
            .windows(MZML_START_TAG.len())
            .any(|window| window == MZML_START_TAG)
        {
            let mzml =
                quick_xml::de::from_reader::<_, MzML<IndexedRun>>(mzml_without_data.as_slice())
                    .context("Failed to parse mzML")?;
            if validate_mzml {
                mzml.validate()?;
            }
            let index = Indexer::create_index(mzml_file, buffer_size)?;
            return Ok(File {
                index,
                reader: mzml_file,
                mzml_element: MzMlElement::MzML(mzml),
            });
        }

        Err(anyhow!("Failed to parse mzML"))
    }

    /// Reads the mzML file with pre generated index.
    /// There is no check if the index is matching the mzML file.
    ///
    /// # Arguments
    /// * `mzml_file` - A mutable reference to a BufRead and Seek object.
    /// * `index` - The index to use for the mzML file.
    /// * `buffer_size` - An optional buffer size for reading the mzML file.
    /// * `validate_mzml` - A boolean flag to validate the mzML file.
    ///
    pub fn read_pre_indexed(
        mzml_file: &mut F,
        index: Index,
        buffer_size: Option<usize>,
        validate_mzml: bool,
    ) -> Result<File<F>> {
        mzml_file.seek(SeekFrom::Start(0))?;

        let mzml_without_data = Reader::get_mzml_without_data(mzml_file, buffer_size)?;

        if mzml_without_data
            .windows(INDEXED_MZML_START_TAG.len())
            .any(|window| window == INDEXED_MZML_START_TAG)
        {
            let indexed_mzml = quick_xml::de::from_reader::<_, IndexedMzML<IndexedRun>>(
                mzml_without_data.as_slice(),
            )
            .context("Failed to parse indexed mzML")?;
            if validate_mzml {
                indexed_mzml.validate()?;
            }

            return Ok(File {
                index,
                reader: mzml_file,
                mzml_element: MzMlElement::IndexedMzML(indexed_mzml),
            });
        }

        if mzml_without_data
            .windows(MZML_START_TAG.len())
            .any(|window| window == MZML_START_TAG)
        {
            let mzml =
                quick_xml::de::from_reader::<_, MzML<IndexedRun>>(mzml_without_data.as_slice())
                    .context("Failed to parse mzML")?;
            if validate_mzml {
                mzml.validate()?;
            }
            return Ok(File {
                index,
                reader: mzml_file,
                mzml_element: MzMlElement::MzML(mzml),
            });
        }

        Err(anyhow!("Failed to parse mzML"))
    }

    /// Returns the mzML file without the the actual spectrum or chromatogram data.
    ///
    /// # Arguments
    /// * `mzml_file` - A mutable reference to a BufRead and Seek object.
    /// * `buffer_size` - An optional buffer size for reading the mzML file.
    ///
    pub fn get_mzml_without_data(mzml_file: &mut F, buffer_size: Option<usize>) -> Result<Vec<u8>> {
        let buffer_size = buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE);
        // get length of bytes
        let len = mzml_file.seek(std::io::SeekFrom::End(0))?;
        let mut current_buffer_size = len.min(buffer_size as u64) as usize;
        // start from top
        mzml_file.seek(std::io::SeekFrom::Start(0))?;

        // create buffer for reading
        let mut buffer: Vec<u8> = vec![0; current_buffer_size];

        // counter for content
        let mut remaining_content = len as usize;

        // create vec for content before the <spectrumList tag
        let mut start_of_mzml: Vec<u8> = Vec::with_capacity(current_buffer_size);
        let mut search_offset = 0;
        loop {
            // Read bytes and add to start_of_mzml
            let read_end = mzml_file
                .read(buffer.as_mut_slice())
                .context("Failed to read next chunk in search for the <run>-tag")?;
            start_of_mzml.extend_from_slice(&buffer[..read_end]);
            // Search for the OPENING_SPECTRUM_LIST_TAG in start_of_mzml starting where the last search ended
            let run_tag_offset = start_of_mzml[search_offset..]
                .windows(OPENING_SPECTRUM_LIST_TAG.len())
                .position(|window| window == OPENING_SPECTRUM_LIST_TAG);
            // If the tag is not found and there is still content to read, continue
            if run_tag_offset.is_none() {
                // set the search offset to the end of the last search - the length of the searched tag in case of overlapping
                search_offset = start_of_mzml.len() - OPENING_SPECTRUM_LIST_TAG.len();
                remaining_content -= buffer.capacity();
                // Adjust buffer size if there is less content remaining
                if remaining_content < current_buffer_size {
                    current_buffer_size = remaining_content;
                    buffer.truncate(current_buffer_size);
                }
                continue;
            }
            start_of_mzml.truncate(search_offset + run_tag_offset.unwrap_or(0));
            break;
        }

        // Remove everything until the last newline for proper indention with the rest of the mzML
        loop {
            if let Some(c) = start_of_mzml.last() {
                if *c != b'\n' {
                    start_of_mzml.pop();
                } else {
                    break;
                }
            }
        }
        let mut end_of_mzml: Vec<u8> = Vec::with_capacity(current_buffer_size);

        // reset and start form the end
        current_buffer_size = len.min(current_buffer_size as u64) as usize;
        remaining_content = len as usize;
        search_offset = 0;
        // Move to the end of the file minus the buffer size
        let mut last_file_pos =
            mzml_file.seek(std::io::SeekFrom::Start(len - current_buffer_size as u64))?;

        loop {
            // read bytes and add to end_of_mzml
            let read_end = mzml_file
                .read(buffer.as_mut_slice())
                .context("Failed to read next chunk in search for the </run>-tag")?;
            buffer[..read_end].as_mut().reverse();
            end_of_mzml.extend_from_slice(&buffer[..read_end]);
            let run_tag_offset = end_of_mzml[search_offset..]
                .windows(CLOSING_RUN_TAG_REV.len())
                .position(|window| window == CLOSING_RUN_TAG_REV);
            // If the tag is not found and there is still content to read, continue
            if run_tag_offset.is_none() && remaining_content > 0 {
                if remaining_content >= buffer.capacity() {
                    remaining_content -= buffer.capacity();
                } else {
                    remaining_content = 0;
                }
                if remaining_content < current_buffer_size {
                    current_buffer_size = remaining_content;
                    buffer.truncate(current_buffer_size);
                }
                search_offset = end_of_mzml.len() - CLOSING_RUN_TAG_REV.len();
                last_file_pos = mzml_file.seek(std::io::SeekFrom::Start(
                    last_file_pos - current_buffer_size as u64,
                ))?;
                continue;
            }
            let mut truncate_index =
                search_offset + run_tag_offset.unwrap_or(0) + CLOSING_RUN_TAG_REV.len();

            // for proper indention we need top add everything until the next newline
            // which should be within the next 10 bytes or so
            // if there are to few bytes left, we read the next 10 bytes
            if truncate_index + 10 > end_of_mzml.len() {
                buffer.truncate(10);
                mzml_file.seek(SeekFrom::Start(last_file_pos - 10))?;
                mzml_file.read_exact(buffer.as_mut_slice())?;
                buffer.reverse();
                end_of_mzml.extend_from_slice(&buffer);
            }

            for i in 0..10 {
                if end_of_mzml[truncate_index + i] == b'\n' {
                    truncate_index += i;
                    break;
                }
            }

            end_of_mzml.truncate(truncate_index);
            end_of_mzml.reverse();
            break;
        }

        Ok([start_of_mzml, end_of_mzml].concat())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Reused test spectrums
    ///
    fn test_spectrum_3865(spectrum: Spectrum) {
        assert_eq!(spectrum.id, "controllerType=0 controllerNumber=1 scan=3865");
        assert_eq!(spectrum.index, 3);
        let precursor_ion = &spectrum.precursor_list.unwrap().precursors[0]
            .selected_ion_list
            .selected_ions[0];
        let mass_to_charge = precursor_ion
            .cv_params
            .iter()
            .find(|cv_param| cv_param.accession == "MS:1000744")
            .unwrap();
        assert_eq!(mass_to_charge.value, "447.346893310547");
    }

    #[test]
    fn test_get_mzml_without_data() {
        let raw_file = std::fs::File::open("test_files/spectra_small.unindexed.mzML").unwrap();
        let mut raw_reader = std::io::BufReader::new(raw_file);
        let expected_string =
            std::fs::read_to_string("test_files/spectra_small.unindexed.no_data.mzML").unwrap();

        let mzml_without_data = Reader::get_mzml_without_data(&mut raw_reader, None).unwrap();
        let mzml_without_data_string = String::from_utf8(mzml_without_data).unwrap();
        assert_eq!(mzml_without_data_string, expected_string);
    }

    #[test]
    fn test_get_indexed_mzml_without_data() {
        let raw_file = std::fs::File::open("test_files/spectra_small.mzML").unwrap();
        let mut raw_reader = std::io::BufReader::new(raw_file);
        let expected_string =
            std::fs::read_to_string("test_files/spectra_small.no_data.mzML").unwrap();

        let mzml_without_data = Reader::get_mzml_without_data(&mut raw_reader, None).unwrap();
        let mzml_without_data_string = String::from_utf8(mzml_without_data).unwrap();
        assert_eq!(mzml_without_data_string, expected_string);
    }

    #[test]
    fn test_reader_mzml() {
        let raw_file = std::fs::File::open("test_files/spectra_small.unindexed.mzML").unwrap();
        let mut raw_reader = std::io::BufReader::new(raw_file);
        let file = Reader::read_indexed(&mut raw_reader, None, false, true).unwrap();
        assert_eq!(file.index.get_spectra().len(), 11);
        assert_eq!(file.index.get_chromatograms().len(), 1);
        assert!(matches!(file.mzml_element, MzMlElement::MzML(_)));
    }

    #[test]
    fn test_reader_indexed_mzml() {
        let raw_file = std::fs::File::open("test_files/spectra_small.mzML").unwrap();
        let mut raw_reader = std::io::BufReader::new(raw_file);
        let file = Reader::read_indexed(&mut raw_reader, None, false, true).unwrap();
        assert_eq!(file.index.get_spectra().len(), 11);
        assert_eq!(file.index.get_chromatograms().len(), 1);
        assert!(matches!(file.mzml_element, MzMlElement::IndexedMzML(_)));
    }

    #[test]
    fn test_reader_indexed_mzml_reindex() {
        let raw_file = std::fs::File::open("test_files/spectra_small.mzML").unwrap();
        let mut raw_reader = std::io::BufReader::new(raw_file);
        let file = Reader::read_indexed(&mut raw_reader, None, true, true).unwrap();
        assert_eq!(file.index.get_spectra().len(), 11);
        assert_eq!(file.index.get_chromatograms().len(), 1);
        assert!(matches!(file.mzml_element, MzMlElement::IndexedMzML(_)));
    }

    #[test]
    fn test_reader_indexed_mzml_with_existing_index() {
        // create buffered reader
        let raw_file = std::fs::File::open("test_files/spectra_small.mzML").unwrap();
        let mut raw_reader = std::io::BufReader::new(raw_file);
        // create index
        let index = Indexer::create_index(&mut raw_reader, None).unwrap();
        // read mzml with pre generated index
        let mut file = Reader::read_pre_indexed(&mut raw_reader, index, None, true).unwrap();
        // get spectrum and check some attributes
        let spectrum = file
            .get_spectrum("controllerType=0 controllerNumber=1 scan=3865")
            .unwrap();
        test_spectrum_3865(spectrum);
    }

    #[test]
    fn test_spectrum_separation_from_mzml() {
        // Read file
        let mut inner_reader = std::io::BufReader::new(
            std::fs::File::open("test_files/spectra_small.unindexed.mzML").unwrap(),
        );
        let mut mzml_file = Reader::read_indexed(&mut inner_reader, None, false, true).unwrap();
        // Extract spectrum
        let xml = mzml_file
            .extract_spectrum("controllerType=0 controllerNumber=1 scan=3865", false)
            .unwrap();
        std::fs::write("./tmp.mzML", &xml).unwrap();
        // Read extracted file, no reindexing but validation
        let mut extracted_inner_reader = std::io::Cursor::new(xml);
        let mut extracted_mzml_file =
            Reader::read_indexed(&mut extracted_inner_reader, None, false, true).unwrap();
        // Get spectrum and check some attributes
        let spectrum = extracted_mzml_file
            .get_spectrum("controllerType=0 controllerNumber=1 scan=3865")
            .unwrap();
        test_spectrum_3865(spectrum);
    }

    #[test]
    fn test_spectrum_separation_from_indexedmzml() {
        // Read file
        let mut inner_reader =
            std::io::BufReader::new(std::fs::File::open("test_files/spectra_small.mzML").unwrap());
        let mut mzml_file = Reader::read_indexed(&mut inner_reader, None, false, true).unwrap();
        // Extract spectrum
        let xml = mzml_file
            .extract_spectrum("controllerType=0 controllerNumber=1 scan=3865", false)
            .unwrap();
        std::fs::write("./tmp.indexed.mzML", &xml).unwrap();
        // Read extracted file, no reindexing but validation
        let mut extracted_inner_reader = std::io::Cursor::new(xml);
        let mut extracted_mzml_file =
            Reader::read_indexed(&mut extracted_inner_reader, None, false, true).unwrap();
        // Get spectrum and check some attributes
        let spectrum = extracted_mzml_file
            .get_spectrum("controllerType=0 controllerNumber=1 scan=3865")
            .unwrap();
        test_spectrum_3865(spectrum);
    }
}
