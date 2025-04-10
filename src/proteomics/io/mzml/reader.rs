use core::fmt::Write;
use std::collections::HashMap;
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
use super::indexer::{get_id_attributes, Indexer};

/// Default buffer size
///
const DEFAULT_BUFFER_SIZE: usize = 1024 * 1000; // 1MB

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

/// MzML file with indexed spectra and chromatograms.
type CompactMzML = (
    Vec<u8>,
    HashMap<String, usize>,
    HashMap<String, usize>,
    bool,
);

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

        let (mzml_without_data, spectrum_offsets, chromatogram_offsets, is_indexed_mzml) =
            Reader::get_mzml_without_data(mzml_file, buffer_size, force_reindex)?;

        if is_indexed_mzml {
            let indexed_mzml = quick_xml::de::from_reader::<_, IndexedMzML<IndexedRun>>(
                mzml_without_data.as_slice(),
            )?;
            if validate_mzml {
                indexed_mzml.validate()?;
            }
            let index = if force_reindex {
                Index::new(spectrum_offsets, chromatogram_offsets)
            } else {
                Index::from(&indexed_mzml.index_list)
            };

            Ok(File {
                index,
                reader: mzml_file,
                mzml_element: MzMlElement::IndexedMzML(indexed_mzml),
            })
        } else {
            let mzml =
                quick_xml::de::from_reader::<_, MzML<IndexedRun>>(mzml_without_data.as_slice())?;
            if validate_mzml {
                mzml.validate()?;
            }
            let index = Index::new(spectrum_offsets, chromatogram_offsets);
            Ok(File {
                index,
                reader: mzml_file,
                mzml_element: MzMlElement::MzML(mzml),
            })
        }
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

        let (mzml_without_data, _, _, is_indexed_mzml) =
            Reader::get_mzml_without_data(mzml_file, buffer_size, false)?;

        if is_indexed_mzml {
            let indexed_mzml = quick_xml::de::from_reader::<_, IndexedMzML<IndexedRun>>(
                mzml_without_data.as_slice(),
            )
            .context("Failed to parse indexed mzML")?;
            if validate_mzml {
                indexed_mzml.validate()?;
            }

            Ok(File {
                index,
                reader: mzml_file,
                mzml_element: MzMlElement::IndexedMzML(indexed_mzml),
            })
        } else {
            let mzml =
                quick_xml::de::from_reader::<_, MzML<IndexedRun>>(mzml_without_data.as_slice())
                    .context("Failed to parse mzML")?;
            if validate_mzml {
                mzml.validate()?;
            }

            Ok(File {
                index,
                reader: mzml_file,
                mzml_element: MzMlElement::MzML(mzml),
            })
        }
    }

    fn push_start_event(content: &mut Vec<u8>, e: &[u8]) {
        content.push(b'<');
        content.extend_from_slice(e);
        content.push(b'>');
    }

    fn push_end_event(content: &mut Vec<u8>, e: &[u8]) {
        content.push(b'<');
        content.push(b'/');
        content.extend_from_slice(e);
        content.push(b'>');
    }

    fn push_empty_event(content: &mut Vec<u8>, e: &[u8]) {
        content.push(b'<');
        content.extend_from_slice(e);
        content.push(b'/');
        content.push(b'>');
    }

    /// Returns the mzML file without the the actual spectrum or chromatogram data.
    ///
    /// # Arguments
    /// * `mzml_file` - A mutable reference to a BufRead and Seek object.
    /// * `buffer_size` - An optional buffer size for reading the mzML file.
    ///
    fn get_mzml_without_data(
        mzml_file: &mut F,
        buffer_size: Option<usize>,
        force_reindex: bool,
    ) -> Result<CompactMzML> {
        mzml_file.seek(std::io::SeekFrom::Start(0))?;
        let mut reader = quick_xml::Reader::from_reader(mzml_file);
        let buffer_size = buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE);
        let mut buffer = Vec::with_capacity(buffer_size);
        let mut is_indexed_mzml = false;
        let mut spectrum_offsets: HashMap<String, usize> = HashMap::new();
        let mut chromatogram_offsets: HashMap<String, usize> = HashMap::new();
        let mut content = Vec::new();
        let mut add_content = true;
        let mut force_reindex = force_reindex;
        loop {
            match reader.read_event_into(&mut buffer) {
                Ok(quick_xml::events::Event::Start(ref e)) => match e.local_name().as_ref() {
                    b"indexedmzML" => {
                        is_indexed_mzml = true;
                        Self::push_start_event(&mut content, e);
                    }
                    b"mzML" => {
                        // if the mzML file is not indexed, we need to index it
                        if !is_indexed_mzml {
                            force_reindex = true;
                        }
                        Self::push_start_event(&mut content, e);
                    }
                    b"spectrumList" => {
                        Self::push_start_event(&mut content, e);
                        add_content = false;
                    }
                    b"chromatogramList" => {
                        Self::push_start_event(&mut content, e);
                        add_content = false;
                    }
                    b"spectrum" => {
                        if force_reindex {
                            spectrum_offsets.insert(
                                get_id_attributes(e)?,
                                reader.buffer_position() as usize - e.len() - 2, // position reports last byte of start tag `>`, mzML wants the first byte `<`
                            );
                        }
                    }
                    b"chromatogram" => {
                        if force_reindex {
                            chromatogram_offsets.insert(
                                get_id_attributes(e)?,
                                reader.buffer_position() as usize - e.len() - 2, // position reports last byte of start tag `>`, mzML wants the first byte `<`
                            );
                        }
                    }
                    _ => {
                        if add_content {
                            Self::push_start_event(&mut content, e);
                        }
                    }
                },
                Ok(quick_xml::events::Event::End(ref e)) => match e.local_name().as_ref() {
                    b"spectrumList" => {
                        Self::push_end_event(&mut content, e);
                        add_content = true;
                    }
                    b"chromatogramList" => {
                        Self::push_end_event(&mut content, e);
                        add_content = true;
                    }
                    _ => {
                        if add_content {
                            Self::push_end_event(&mut content, e);
                        }
                    }
                },
                Ok(quick_xml::events::Event::Empty(ref e)) => {
                    if add_content {
                        Self::push_empty_event(&mut content, e);
                    }
                }
                Ok(quick_xml::events::Event::Text(ref e)) => {
                    if add_content {
                        content.extend_from_slice(e);
                    }
                }
                Ok(quick_xml::events::Event::Decl(ref e)) => {
                    if add_content {
                        content.push(b'<');
                        content.push(b'?');
                        content.extend_from_slice(e);
                        content.push(b'?');
                        content.push(b'>');
                    }
                }
                Ok(quick_xml::events::Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
        }
        Ok((
            content,
            spectrum_offsets,
            chromatogram_offsets,
            is_indexed_mzml,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Check if the spectrum with id `controllerType=0 controllerNumber=1 scan=3865` has the correct
    /// data
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

    /// Get the mzML file without the data
    /// and check if the content is correct.
    #[test]
    fn test_get_mzml_without_data() {
        let raw_file = std::fs::File::open("test_files/spectra_small.unindexed.mzML").unwrap();
        let mut raw_reader = std::io::BufReader::new(raw_file);
        let expected_string =
            std::fs::read_to_string("test_files/spectra_small.unindexed.no_data.mzML").unwrap();

        let (mzml_without_data, spectrum_offsets, chromatogram_offsets, is_indexed_mzml) =
            Reader::get_mzml_without_data(&mut raw_reader, None, false).unwrap();
        let mzml_without_data_string = String::from_utf8(mzml_without_data).unwrap();
        // check the content
        assert_eq!(mzml_without_data_string, expected_string);
        // should be 11 chromatogram, and proofs the reindexing was forced as the file it is not indexed
        assert_eq!(spectrum_offsets.len(), 11);
        // should be 1 chromatogram, and proofs the reindexing was forced as the file it is not indexed
        assert_eq!(chromatogram_offsets.len(), 1);
        // should be false
        assert!(!is_indexed_mzml);
    }

    /// Get the indexed mzML file without the data
    /// and check if the content is correct.
    #[test]
    fn test_get_indexed_mzml_without_data() {
        let raw_file = std::fs::File::open("test_files/spectra_small.mzML").unwrap();
        let mut raw_reader = std::io::BufReader::new(raw_file);
        let expected_string =
            std::fs::read_to_string("test_files/spectra_small.no_data.mzML").unwrap();

        let (mzml_without_data, spectrum_offsets, chromatogram_offsets, is_indexed_mzml) =
            Reader::get_mzml_without_data(&mut raw_reader, None, false).unwrap();
        let mzml_without_data_string = String::from_utf8(mzml_without_data).unwrap();
        // check the content
        assert_eq!(mzml_without_data_string, expected_string);
        // should be 0, no reindexing was forced as the file it is indexed
        assert_eq!(spectrum_offsets.len(), 0);
        // should be 0 no reindexing was forced as the file it is indexed
        assert_eq!(chromatogram_offsets.len(), 0);
        // should be true
        assert!(is_indexed_mzml);
    }

    /// Test the reader with an unindexed mzML file
    /// and check if the content is correct.
    ///
    /// # Arguments
    /// * `force_reindex` - A boolean flag to force reindexing of the mzML file.
    ///
    fn test_reader_mzml(force_reindex: bool) {
        let raw_file = std::fs::File::open("test_files/spectra_small.unindexed.mzML").unwrap();
        let mut raw_reader = std::io::BufReader::new(raw_file);
        let file = Reader::read_indexed(&mut raw_reader, None, false, force_reindex).unwrap();
        assert_eq!(file.index.get_spectra().len(), 11);
        assert_eq!(file.index.get_chromatograms().len(), 1);
        assert!(matches!(file.mzml_element, MzMlElement::MzML(_)));
    }

    /// Test the reader with an indexed mzML file
    /// and check if the content is correct.
    /// Reindex is set to false, but should be forced to true
    /// as the file is not indexed.
    ///
    #[test]
    fn test_reader_mzml_no_reindex() {
        test_reader_mzml(false);
    }

    /// Test the reader with an indexed mzML file
    /// and check if the content is correct.
    /// Reindex is set to true, result should be the same as
    /// `test_reader_mzml_no_reindex`
    ///
    #[test]
    fn test_reader_mzml_with_reindex() {
        test_reader_mzml(true);
    }

    /// Test the reader with an indexed mzML file
    /// and check if the content is correct.
    ///
    /// # Arguments
    /// * `force_reindex` - A boolean flag to force reindexing of the mzML file.
    ///
    fn test_reader_indexed_mzml(force_reindex: bool) {
        let raw_file = std::fs::File::open("test_files/spectra_small.mzML").unwrap();
        let mut raw_reader = std::io::BufReader::new(raw_file);
        let file = Reader::read_indexed(&mut raw_reader, None, false, force_reindex).unwrap();
        assert_eq!(file.index.get_spectra().len(), 11);
        assert_eq!(file.index.get_chromatograms().len(), 1);
        assert!(matches!(file.mzml_element, MzMlElement::IndexedMzML(_)));
    }

    /// Test the reader with an indexed mzML file
    /// and check if the content is correct.
    ///
    #[test]
    fn test_reader_indexed_mzml_no_reindex() {
        test_reader_indexed_mzml(false);
    }

    /// Test the reader with an indexed mzML file
    /// and check if the content is correct.
    /// Reindex is set to true, result should be the same as
    /// `test_reader_indexed_mzml_no_reindex`
    #[test]
    fn test_reader_indexed_mzml_with_reindex() {
        test_reader_indexed_mzml(true);
    }

    /// Test the reader with an pre indexed indexed mzML file
    /// and check if the file struct is correctly assembled
    /// and data is accessible
    ///
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

    /// Test the reader with an pre indexed mzML file
    /// and check if the file struct is correctly assembled
    /// and data is accessible
    ///
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

    /// Test the spectrum separation from an indexed mzML file
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
