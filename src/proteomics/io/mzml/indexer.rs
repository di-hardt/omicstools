use std::collections::HashMap;
use std::io::prelude::*;

use anyhow::bail;
use anyhow::Result;

use crate::proteomics::io::mzml::index::Index;

/// Start of spectrum tag
const SPECTRUM_START_TAG: &[u8] = b"<spectrum ";

/// End of spectrum tag
const SPECTRUM_END_TAG: &[u8] = b"</spectrum>";

/// Start of spectrum tag
const CHROMATOGRAM_START_TAG: &[u8] = b"<chromatogram ";

/// End of spectrum tag
const CHROMATOGRAM_END_TAG: &[u8] = b"</chromatogram>";

/// Start of spectrum ID tag
const SPECTRUM_ID_START: &[u8] = b"id=\"";

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
    buffer: Vec<u8>,
    content: Vec<u8>,
    file_position: usize,
}

impl<'a, F> Indexer<'a, F>
where
    F: BufRead + Seek,
{
    pub fn new(mzml_file: &'a mut F, buffer_size: Option<usize>) -> Self {
        Self {
            mzml_file,
            buffer: vec![0; buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE)],
            content: Vec::with_capacity(buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE)),
            file_position: 0,
        }
    }

    /// Read the next chunk of the file into the buffer.
    ///
    fn read_chunk(&mut self) -> Result<usize, std::io::Error> {
        let num_bytes = self.mzml_file.read(&mut self.buffer)?;
        self.file_position += num_bytes;
        self.content.extend_from_slice(&self.buffer[..num_bytes]);
        Ok(num_bytes)
    }

    /// Find index of closing tag
    ///
    /// # Arguments
    /// * `search_offset` - Offset to start searching from
    /// * `search_for` - Tag to search for
    ///
    fn find_closing_tag(&mut self, mut search_offset: usize, search_for: &[u8]) -> Result<usize> {
        loop {
            if let Some(position) = self.content[search_offset..]
                .windows(search_for.len())
                .position(|window| window == search_for)
            {
                return Ok(search_offset + position + search_for.len());
            }
            search_offset = self.content.len() - search_for.len();
            self.read_chunk()?;
            if self.content.is_empty() {
                bail!("Closing tag not found.");
            }
        }
    }

    /// Searches and sets the spectrum offsets.
    ///
    /// # Arguments
    /// * `plain_spectrum` - Plain spectrum string
    ///
    fn get_spectrum_id(plain_spectrum: &[u8]) -> Result<String> {
        let spec_id_start_idx: usize = match plain_spectrum
            .windows(SPECTRUM_ID_START.len())
            .position(|window| window == SPECTRUM_ID_START)
        {
            Some(position) => position + SPECTRUM_ID_START.len(),
            None => bail!("Spectrum ID attribute start not found."),
        };
        let spec_id_end_idx: usize = match plain_spectrum[spec_id_start_idx..]
            .windows(ATTRIBUTE_END.len())
            .position(|window| window == ATTRIBUTE_END)
        {
            Some(position) => spec_id_start_idx + position,
            None => bail!("Spectrum ID attribute end not found."),
        };
        Ok(
            String::from_utf8_lossy(&plain_spectrum[spec_id_start_idx..spec_id_end_idx])
                .to_string(),
        )
    }

    fn foo(
        &mut self,
        start_tag: &[u8],
        end_tag: &[u8],
        search_offset: &mut usize,
        offsets: &mut HashMap<String, usize>,
    ) -> Result<bool> {
        if let Some(position) = self.content[*search_offset..]
            .windows(start_tag.len())
            .position(|window| window == start_tag)
        {
            // Relative offset of the start tag in loaded content
            let relative_start_offset = *search_offset + position;
            // Absolut position of the start tag in the file
            let absolut_start_offset =
                self.file_position - self.content.len() + relative_start_offset;
            // Relative position of the end tag
            let relative_end_offset = self.find_closing_tag(relative_start_offset, end_tag)?;
            // Tag ID
            let tag_id =
                Self::get_spectrum_id(&self.content[relative_start_offset..relative_end_offset])?;
            // Add offset to the map
            offsets.insert(tag_id, absolut_start_offset);
            // Free up some memory by removing the content up to the end tag
            drop(self.content.drain(..relative_end_offset));
            // reset search offset
            *search_offset = 0;
            return Ok(true);
        }
        Ok(false)
    }

    /// Returns the spectrum offsets, beginning with '<spectrum ' and ending with </spectrum>.
    ///
    fn create_idx(&mut self) -> Result<Index> {
        self.mzml_file.seek(std::io::SeekFrom::Start(0))?;
        let mut spectra_offsets: HashMap<String, usize> = HashMap::new();
        let mut chromatogram_offsets: HashMap<String, usize> = HashMap::new();
        let mut search_offset = 0;

        loop {
            let num_bytes = self.read_chunk()?;
            if self.foo(
                SPECTRUM_START_TAG,
                SPECTRUM_END_TAG,
                &mut search_offset,
                &mut spectra_offsets,
            )? {
                continue;
            }

            if self.foo(
                CHROMATOGRAM_START_TAG,
                CHROMATOGRAM_END_TAG,
                &mut search_offset,
                &mut chromatogram_offsets,
            )? {
                continue;
            }

            search_offset = self.content.len() - CHROMATOGRAM_END_TAG.len();

            if num_bytes == 0 {
                break;
            }
        }
        Ok(Index::new(spectra_offsets, chromatogram_offsets))
    }

    /// Creates a file index for the given mzML file.
    ///
    /// # Arguments
    /// * `reader`- Open reader.
    /// * `chunk_size` - Size of the chunks to read from the file.
    ///
    pub fn create_index(reader: &'a mut F, chunk_size: Option<usize>) -> Result<Index> {
        Self::new(reader, chunk_size).create_idx()
    }
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
    fn test_index_creation() {
        let file_path = Path::new("./test_files/spectra_small.mzML");
        let mut buf_reader = BufReader::new(File::open(file_path).unwrap());

        let index = Indexer::create_index(&mut buf_reader, None).unwrap();

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
}
