// 3rd party imports
use anyhow::{bail, Context, Result};
use base64::prelude::*;
use flate2::read::ZlibDecoder;
use quick_xml::events::Event as XmlEvent;
use quick_xml::Reader as XmlReader;
use std::io::prelude::*;

// internal imports
use crate::mass_spectrometry::spectrum::{SimpleMsNSpectrum, SimplePrecursor, SimpleSpectrum};
use crate::tools::quick_xml::{
    get_attributes as xml_get_attributes, get_value_of_attribute as xml_get_value_of_attribute,
    AttributeNotFound as XmlAttributeNotFound,
};

/// Enum for different types of spectra
pub enum Spectrum {
    Ms1Spectrum(SimpleSpectrum),
    MsNSpectrum(SimpleMsNSpectrum),
}

/// Type of binary data
enum BinaryDataTarget {
    MZ,
    Intensity,
    _Time, // Defined in mzML but not stored yet anywhere
}

/// Compression type of binary data
enum BinaryDataCompression {
    Zlib,
    None,
}

enum BinaryDataValueType {
    F64,
    F32,
}

// Helper struct for dealing with binary data arrays
struct BinaryDataArray {
    pub data: String,
    pub compression: BinaryDataCompression,
    pub target: BinaryDataTarget,
    pub value_type: BinaryDataValueType,
}

impl BinaryDataArray {
    fn new() -> Self {
        Self {
            data: String::new(),
            compression: BinaryDataCompression::None,
            target: BinaryDataTarget::MZ,
            value_type: BinaryDataValueType::F64,
        }
    }

    fn deflate_data(&self) -> Result<Vec<f64>> {
        let binary_data: Vec<u8> = match self.compression {
            BinaryDataCompression::Zlib => {
                let decoded_data = BASE64_STANDARD.decode(&self.data)?;
                let mut deflated_buffer: Vec<u8> = Vec::new();
                let mut decoder = ZlibDecoder::new(decoded_data.as_slice());
                decoder.read_to_end(&mut deflated_buffer)?;
                deflated_buffer
            }
            BinaryDataCompression::None => BASE64_STANDARD.decode(&self.data)?,
        };
        match self.value_type {
            BinaryDataValueType::F32 => {
                if binary_data.len() % 4 != 0 {
                    bail!("Binary data array is not a multiple of 4");
                }
                Ok(binary_data
                    .chunks(4)
                    .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()) as f64)
                    .collect())
            }
            BinaryDataValueType::F64 => {
                if binary_data.len() % 8 != 0 {
                    bail!("Binary data array is not a multiple of 8");
                }
                Ok(binary_data
                    .chunks(8)
                    .map(|chunk| f64::from_le_bytes(chunk.try_into().unwrap()))
                    .collect())
            }
        }
    }
}

/// Helper struct for building a new SimplePrecursor
///
struct NewSimplePrecursor {
    pub parent_id: String,
    /// (mz, offset lower, offset upper)
    pub isolation_windows: Option<(f64, f64, f64)>,
    /// (mz, charge)
    pub ions: Vec<(f64, Vec<u8>)>,
    /// Accession
    pub activation: (String, f64),
}

impl NewSimplePrecursor {
    fn new() -> Self {
        Self {
            parent_id: String::new(),
            isolation_windows: None,
            ions: Vec::new(),
            activation: (String::new(), 0.0),
        }
    }
}

impl Into<SimplePrecursor> for NewSimplePrecursor {
    fn into(self) -> SimplePrecursor {
        SimplePrecursor::new(
            self.parent_id,
            self.isolation_windows,
            self.ions,
            self.activation,
        )
    }
}

pub struct Reader {}

impl Reader {
    pub fn parse_spectrum_xml(xml: &[u8]) -> Result<Spectrum> {
        // Simple spec
        let mut ms_level: u8 = 0;
        let mut mz: Vec<f64> = Vec::new();
        let mut intensity: Vec<f64> = Vec::new();
        let mut spec_id: String = String::new();

        // MsN spec
        let mut precursors: Vec<SimplePrecursor> = Vec::new();

        // Temporary variables
        let mut current_binary_data_array: BinaryDataArray = BinaryDataArray::new();
        let mut current_precursor: NewSimplePrecursor = NewSimplePrecursor::new();
        let mut current_isolation_windows = (0.0, 0.0, 0.0);
        let mut current_selected_ion: f64 = -1.0;
        let mut current_selected_ion_charges: Vec<u8> = Vec::new();

        let mut xml_reader = XmlReader::from_str(std::str::from_utf8(xml)?);
        let mut tag_path: Vec<u8> = Vec::with_capacity(100);
        loop {
            match xml_reader.read_event()? {
                XmlEvent::Eof => break,
                XmlEvent::Start(event) => {
                    tag_path.extend(b":");
                    tag_path.extend(event.local_name().as_ref());
                    match event.local_name().as_ref() {
                        b"spectrum" => {
                            spec_id =
                                xml_get_value_of_attribute(b"id", &xml_get_attributes(&event)?)?;
                        }
                        b"precursor" => {
                            let attributes = xml_get_attributes(&event)?;

                            current_precursor.parent_id =
                                match xml_get_value_of_attribute(b"spectrumRef", &attributes) {
                                    Ok(val) => val,
                                    Err(err) => {
                                        if err.downcast_ref::<XmlAttributeNotFound>().is_some() {
                                            String::new()
                                        } else {
                                            bail!(err)
                                        }
                                    }
                                };
                            if !current_precursor.parent_id.is_empty() {
                                continue;
                            }

                            // if spectrumRef is not found, there should be a sourceFileRef and a externalSpectrumID
                            // for now we put them together as `sourceFileRef // externalSpectrumID``
                            current_precursor.parent_id =
                                xml_get_value_of_attribute(b"sourceFileRef", &attributes)?;

                            if current_precursor.parent_id.is_empty() {
                                bail!("No spectrumRef or sourceFileRef found for precursor");
                            }

                            current_precursor.parent_id.push_str(" // ");

                            current_precursor.parent_id.push_str(
                                xml_get_value_of_attribute::<String>(
                                    b"externalSpectrumID",
                                    &attributes,
                                )?
                                .as_str(),
                            );
                        }
                        _ => {}
                    }
                }
                XmlEvent::Empty(event) => match event.local_name().as_ref() {
                    b"cvParam" => match tag_path.as_slice() {
                        b":spectrum" => {
                            let attributes = xml_get_attributes(&event)?;

                            let accession: String =
                                xml_get_value_of_attribute(b"accession", &attributes)?;

                            if accession == "MS:1000511" {
                                ms_level = match attributes
                                    .iter()
                                    .find(|attr| attr.key.as_ref() == b"value")
                                {
                                    Some(attr) => {
                                        String::from(attr.unescape_value()?).parse::<u8>()?
                                    }
                                    None => bail!("cvParam for MS level has no value"),
                                };
                            }
                        }
                        b":spectrum:precursorList:precursor:isolationWindow" => {
                            let attributes = xml_get_attributes(&event)?;

                            let accession: Vec<u8> =
                                xml_get_value_of_attribute(b"accession", &attributes)?;

                            match accession.as_slice() {
                                b"MS:1000827" => {
                                    current_isolation_windows.0 =
                                        xml_get_value_of_attribute::<String>(b"value", &attributes)
                                            .context("cvParam MS:1000827 ion has no value")?
                                            .parse()?;
                                }
                                b"MS:1000828" => {
                                    current_isolation_windows.1 =
                                        xml_get_value_of_attribute::<String>(b"value", &attributes)
                                            .context("cvParam MS:1000828 ion has no value")?
                                            .parse()?;
                                }
                                b"MS:1000829" => {
                                    current_isolation_windows.2 =
                                        xml_get_value_of_attribute::<String>(b"value", &attributes)
                                            .context("cvParam MS:1000828 ion has no value")?
                                            .parse()?;
                                }
                                _ => {}
                            }
                        }
                        b":spectrum:precursorList:precursor:selectedIonList:selectedIon" => {
                            let attributes = xml_get_attributes(&event)?;

                            let accession: Vec<u8> =
                                xml_get_value_of_attribute(b"accession", &attributes)?;

                            match accession.as_slice() {
                                b"MS:1000744" => {
                                    current_selected_ion =
                                        xml_get_value_of_attribute::<String>(b"value", &attributes)
                                            .context("cvParam MS:1000744 ion has no value")?
                                            .parse()?;
                                }
                                b"MS:1000041" => {
                                    current_selected_ion_charges.push(
                                        xml_get_value_of_attribute::<String>(b"value", &attributes)
                                            .context("cvParam MS:1000041 ion has no value")?
                                            .parse()?,
                                    );
                                }
                                _ => {}
                            }
                        }
                        b":spectrum:precursorList:precursor:activation" => {
                            // TODO: Parse activation method. Since the collision method is just a cvParam next to several others
                            // where the accession is a SPECIFIC accession for on of the supported methods,
                            // we need to add a table to look up wich accession actual refers to the collision method
                            // Best include the complete table ontology

                            let attributes = xml_get_attributes(&event)?;

                            let accession: Vec<u8> =
                                xml_get_value_of_attribute(b"accession", &attributes)?;

                            match accession.as_slice() {
                                b"MS:1000045" => {
                                    current_precursor.activation.1 = xml_get_value_of_attribute::<
                                        String,
                                    >(
                                        b"value", &attributes
                                    )?
                                    .parse()?;
                                }
                                _ => {}
                            }
                        }
                        b":spectrum:binaryDataArrayList:binaryDataArray" => {
                            let accession: Vec<u8> = xml_get_value_of_attribute(
                                b"accession",
                                &xml_get_attributes(&event)?,
                            )?;
                            match accession.as_slice() {
                                b"MS:1000574" => {
                                    current_binary_data_array.compression =
                                        BinaryDataCompression::Zlib;
                                }
                                b"MS:1000576" => {
                                    current_binary_data_array.compression =
                                        BinaryDataCompression::None;
                                }
                                b"MS:1000514" => {
                                    current_binary_data_array.target = BinaryDataTarget::MZ;
                                }
                                b"MS:1000515" => {
                                    current_binary_data_array.target = BinaryDataTarget::Intensity;
                                }
                                b"MS:1000521" => {
                                    current_binary_data_array.value_type = BinaryDataValueType::F32;
                                }
                                b"MS:1000523" => {
                                    current_binary_data_array.value_type = BinaryDataValueType::F64;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
                XmlEvent::Text(event) => {
                    if tag_path == b":spectrum:binaryDataArrayList:binaryDataArray:binary" {
                        current_binary_data_array.data = event.unescape()?.into_owned();
                    }
                }
                XmlEvent::End(event) => {
                    tag_path.truncate(tag_path.len() - event.local_name().as_ref().len() - 1);
                    match event.local_name().as_ref() {
                        b"precursor" => {
                            precursors.push(current_precursor.into());
                            current_precursor = NewSimplePrecursor::new();
                        }
                        b"isolationWindow" => {
                            current_precursor.isolation_windows = Some(current_isolation_windows);
                            current_isolation_windows = (0.0, 0.0, 0.0);
                        }
                        b"selectedIon" => {
                            if current_selected_ion >= 0.0 {
                                current_precursor
                                    .ions
                                    .push((current_selected_ion, current_selected_ion_charges));
                            }
                            current_selected_ion = -1.0;
                            current_selected_ion_charges = Vec::new();
                        }
                        b"binaryDataArray" => match current_binary_data_array.target {
                            BinaryDataTarget::Intensity => {
                                intensity = current_binary_data_array.deflate_data()?;
                            }
                            BinaryDataTarget::MZ => {
                                mz = current_binary_data_array.deflate_data()?;
                            }
                            _ => {
                                bail!("Binary data array has no data type")
                            }
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        if ms_level == 1 {
            Ok(Spectrum::Ms1Spectrum(SimpleSpectrum::new(
                spec_id, ms_level, mz, intensity,
            )))
        } else {
            Ok(Spectrum::MsNSpectrum(SimpleMsNSpectrum::new(
                spec_id, ms_level, mz, intensity, precursors,
            )))
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::{
        mass_spectrometry::spectrum::{
            MsNSpectrum as MsNSpectrumTrait, Precursor as PrecursorTrait, Spectrum as SpectrumTrait,
        },
        proteomics::io::mzml::{indexed_reader::IndexedReader, indexer::Indexer},
    };

    use super::*;

    #[test]
    fn test_spectrum_parsing() {
        let index =
            Indexer::create_index(Path::new("test_files/spectra_small.mzML"), None).unwrap();

        let mut mzml_reader =
            IndexedReader::new(Path::new("test_files/spectra_small.mzML"), &index).unwrap();

        let spectrum_xml = mzml_reader
            .get_raw_spectrum("controllerType=0 controllerNumber=1 scan=2814")
            .unwrap();

        let spectrum = Reader::parse_spectrum_xml(spectrum_xml.as_slice()).unwrap();

        match spectrum {
            Spectrum::Ms1Spectrum(_) => {
                panic!("Expected MSn spectrum, got MS1 spectrum")
            }
            Spectrum::MsNSpectrum(spec) => {
                assert_eq!(
                    spec.get_id(),
                    "controllerType=0 controllerNumber=1 scan=2814"
                );

                assert_eq!(spec.get_ms_level(), 2);
                assert_eq!(spec.get_mz().len(), spec.get_intensity().len());

                assert_eq!(spec.get_precursors().len(), 1);

                let precursor = spec.get_precursors().first().unwrap();

                assert_eq!(
                    precursor.get_parent_id(),
                    "controllerType=0 controllerNumber=1 scan=2813"
                );

                // TODO: Test external referenced spectrum

                assert_eq!(
                    precursor.get_isolation_windows().unwrap(),
                    (562.916259765625, 0.800000011921, 0.800000011922)
                );

                assert_eq!(precursor.get_ions().len(), 1);

                let ion = precursor.get_ions().first().unwrap();

                assert_eq!(ion.0, 562.916259765625);
                assert_eq!(ion.1, vec![2]);

                assert_eq!(precursor.get_activation().1, 28.0);
            } // TODO: Get a MS1 spectrum to test
        }
    }
}
