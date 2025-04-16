use std::collections::HashMap;

use anyhow::{anyhow, bail, Context, Error, Result};

use crate::proteomics::io::mzml::elements::{
    has_cv_params::HasCvParams, spectrum::Spectrum as MzMlSpectrum,
};
use crate::proteomics::ontology::Ontology;

/// PSI ontology accession for dissociation method
const DISSOCIATION_METHOD_ACCESSION: &str = "MS:1000044";

/// PSI ontology accession for precursor activation attribute
const PRECURSOR_ACTIVATION_ATTRIBUTES_ACCESSION: &str = "MS:1000510";

/// Trait defining a basic spectrum
/// Should at least contain mz and intensity vectors, the ms level and identifier.
///
pub trait Spectrum {
    /// Returns the spectrum id
    ///
    fn get_id(&self) -> &String;

    /// Returns the MsLevel
    ///
    fn get_ms_level(&self) -> u8;

    /// Returns the mz values
    ///
    fn get_mz(&self) -> &Vec<f64>;

    /// Returns the intensity values
    ///
    fn get_intensity(&self) -> &Vec<f64>;
}

/// Trait defining a n-level spectrum with a parent ID, precursor m/z and precursor charge
///
pub trait MsNSpectrum<P>: Spectrum
where
    P: Precursor,
{
    /// Returns the precursors
    ///
    fn get_precursors(&self) -> &Vec<P>;
}

/// Trait defining a precursor
///
pub trait Precursor {
    /// Returns the parent spectrum ID
    fn get_parent_id(&self) -> &String;

    /// Returns the isolation windows (mz, offset lower, offset upper)
    fn get_isolation_windows(&self) -> &Option<(f64, f64, f64)>;

    /// Returns the ions (mz, charge)
    fn get_ions(&self) -> &Option<Vec<(f64, Vec<u8>)>>;

    /// Returns the activation method (methods, attributes)
    fn get_activation(&self) -> &(Vec<String>, HashMap<String, String>);
}

/// Simplest version of a Precursor (for now)
///
pub struct SimplePrecursor {
    parent_id: String,
    /// (mz, offset lower, offset upper)
    isolation_windows: Option<(f64, f64, f64)>,
    /// (mz, charge)
    ions: Option<Vec<(f64, Vec<u8>)>>,
    /// Accession
    activation: (Vec<String>, HashMap<String, String>),
}

impl SimplePrecursor {
    pub fn new(
        parent_id: String,
        isolation_windows: Option<(f64, f64, f64)>,
        ions: Option<Vec<(f64, Vec<u8>)>>,
        activation: (Vec<String>, HashMap<String, String>),
    ) -> Self {
        Self {
            parent_id,
            isolation_windows,
            ions,
            activation,
        }
    }
}

impl Precursor for SimplePrecursor {
    fn get_parent_id(&self) -> &String {
        &self.parent_id
    }

    fn get_isolation_windows(&self) -> &Option<(f64, f64, f64)> {
        &self.isolation_windows
    }

    fn get_ions(&self) -> &Option<Vec<(f64, Vec<u8>)>> {
        &self.ions
    }

    fn get_activation(&self) -> &(Vec<String>, HashMap<String, String>) {
        &self.activation
    }
}

/// Simplest version of a Spectrum
/// Contains mz, intensity, id and ms_level
/// Depending on the file format there might be a whole lot more information available.
///
pub struct SimpleSpectrum {
    id: String,
    ms_level: u8,
    mz: Vec<f64>,
    intensity: Vec<f64>,
}

impl SimpleSpectrum {
    pub fn new(id: String, ms_level: u8, mz: Vec<f64>, intensity: Vec<f64>) -> Self {
        Self {
            id,
            ms_level,
            mz,
            intensity,
        }
    }
}

impl Spectrum for SimpleSpectrum {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_ms_level(&self) -> u8 {
        self.ms_level
    }

    fn get_mz(&self) -> &Vec<f64> {
        &self.mz
    }

    fn get_intensity(&self) -> &Vec<f64> {
        &self.intensity
    }
}

/// Simplest version of an MsNSpectrum
/// Contains mz, intensity, id, ms_level, precursor_mz and precursor_charge
/// Depending on the file format there might be a whole lot more information available.
/// But this should be the bare minimum to work with
///
pub struct SimpleMsNSpectrum {
    id: String,
    ms_level: u8,
    mz: Vec<f64>,
    intensity: Vec<f64>,
    precursors: Vec<SimplePrecursor>,
}

impl SimpleMsNSpectrum {
    pub fn new(
        id: String,
        ms_level: u8,
        mz: Vec<f64>,
        intensity: Vec<f64>,
        precursors: Vec<SimplePrecursor>,
    ) -> Self {
        Self {
            id,
            ms_level,
            mz,
            intensity,
            precursors,
        }
    }
}

impl Spectrum for SimpleMsNSpectrum {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_ms_level(&self) -> u8 {
        self.ms_level
    }

    fn get_mz(&self) -> &Vec<f64> {
        &self.mz
    }

    fn get_intensity(&self) -> &Vec<f64> {
        &self.intensity
    }
}

impl MsNSpectrum<SimplePrecursor> for SimpleMsNSpectrum {
    fn get_precursors(&self) -> &Vec<SimplePrecursor> {
        &self.precursors
    }
}

impl TryFrom<MzMlSpectrum> for SimpleMsNSpectrum {
    type Error = anyhow::Error;

    fn try_from(spectrum: MzMlSpectrum) -> Result<Self> {
        let ms_level = match spectrum.get_ms_level() {
            Some(ms_level) => ms_level,
            None => {
                bail!("Spectrum {} - Failed to get ms level", spectrum.id)
            }
        };

        let mz_binary_array = match spectrum.binary_data_array_list.get_mz_array() {
            Ok(mz_binary_array) => mz_binary_array,
            Err(err) => {
                bail!(
                    "Spectrum {} - Failed to get mz binary data array: {}",
                    spectrum.id,
                    err
                )
            }
        };

        let mz_vec = match mz_binary_array.deflate_data() {
            Ok(mz_vec) => mz_vec,
            Err(err) => {
                bail!(
                    "Spectrum {} - Failed to deflate mz binary data array: {}",
                    spectrum.id,
                    err
                )
            }
        };

        let intensity_binary_array = match spectrum.binary_data_array_list.get_intensity_array() {
            Ok(intensity_binary_array) => intensity_binary_array,
            Err(err) => {
                bail!(
                    "Spectrum {} - Failed to get intensity binary data array: {}",
                    spectrum.id,
                    err
                )
            }
        };

        let intensity_vec = match intensity_binary_array.deflate_data() {
            Ok(intensity_vec) => intensity_vec,
            Err(err) => {
                bail!(
                    "Spectrum {} - Failed to deflate intensity binary data array: {}",
                    spectrum.id,
                    err
                )
            }
        };

        let mut precursors: Vec<SimplePrecursor> = Vec::new();

        if let Some(ref precursor_list) = spectrum.precursor_list {
            let precs: Vec<SimplePrecursor> = precursor_list
                .precursors
                .iter()
                .map(|precursor| {
                    let isolation_windows: Option<(f64, f64, f64)> =
                        match precursor.isolation_window.as_ref() {
                            Some(window) => Ok::<_, Error>(Some((
                                match window.get_cv_param("MS:1000827").first() {
                                    Some(cv_param) => cv_param.value.parse::<f64>().context(
                                        "Error when parsing isolation window target m/z",
                                    )?,
                                    None => 0.0,
                                },
                                match window.get_cv_param("MS:1000828").first() {
                                    Some(cv_param) => cv_param.value.parse::<f64>().context(
                                        "Error when parsing isolation window lower offset",
                                    )?,
                                    None => 0.0,
                                },
                                match window.get_cv_param("MS:1000829").first() {
                                    Some(cv_param) => cv_param.value.parse::<f64>().context(
                                        "Error when parsing isolation window upper offset",
                                    )?,
                                    None => 0.0,
                                },
                            ))),
                            None => Ok(None),
                        }?;

                    let ions: Option<Vec<(f64, Vec<u8>)>> =
                        match precursor.selected_ion_list.as_ref() {
                            Some(ions_list) => {
                                let ions_converted = ions_list
                                    .selected_ions
                                    .iter()
                                    .map(|ion| {
                                        let mz: f64 = ion
                                            .get_cv_param("MS:1000744")
                                            .first()
                                            .ok_or_else(|| {
                                                anyhow!("Spectrum does not have selected ion m/z")
                                            })?
                                            .value
                                            .parse()
                                            .context("Cannot parse selected ion m/z to f64")?;
                                        // // Select charge states states
                                        let mut charges: Vec<u8> = ion
                                            .get_cv_param("MS:1000041")
                                            .iter()
                                            .map(|cv_param| {
                                                cv_param
                                                    .value
                                                    .parse::<u8>()
                                                    .context("Error when parsing charge state")
                                            })
                                            .collect::<Result<Vec<u8>>>()?;
                                        // add possible charge states
                                        let possible_charges: Vec<u8> = ion
                                            .get_cv_param("MS:1000633")
                                            .iter()
                                            .map(|cv_param| {
                                                cv_param.value.parse::<u8>().context(
                                                    "Error when parsing possible charge state",
                                                )
                                            })
                                            .collect::<Result<Vec<u8>>>()?;
                                        charges.append(&mut possible_charges.clone());
                                        Ok((mz, charges))
                                    })
                                    .collect::<Result<Vec<_>>>()?;

                                Ok::<_, Error>(Some(ions_converted))
                            }
                            None => Ok(None),
                        }?;

                    let activation = {
                        let method_children =
                            Ontology::get_children_of(DISSOCIATION_METHOD_ACCESSION)?;

                        let attributes_children =
                            Ontology::get_children_of(PRECURSOR_ACTIVATION_ATTRIBUTES_ACCESSION)?;

                        let methods: Vec<String> = method_children
                            .iter()
                            .flat_map(|accession| {
                                precursor
                                    .activation
                                    .get_cv_param(accession)
                                    .iter()
                                    .map(|cv_param| cv_param.value.clone())
                                    .collect::<Vec<_>>()
                            })
                            .collect();

                        let attributes: HashMap<String, String> = attributes_children
                            .iter()
                            .flat_map(|accession| {
                                precursor
                                    .activation
                                    .get_cv_param(accession)
                                    .iter()
                                    .map(|cv_param| {
                                        (cv_param.accession.clone(), cv_param.value.clone())
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .collect();

                        Ok::<_, Error>((methods, attributes))
                    }?;

                    Ok(SimplePrecursor::new(
                        precursor.spectrum_ref.clone(),
                        isolation_windows,
                        ions,
                        activation,
                    ))
                })
                .collect::<Result<Vec<_>>>()?;
            precursors = precs
        }

        Ok(SimpleMsNSpectrum::new(
            spectrum.id,
            ms_level,
            mz_vec,
            intensity_vec,
            precursors,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;
    use crate::proteomics::io::mzml::reader::Reader as MzMlReader;

    #[test]
    fn test_from_mzml_spectrum() {
        // read spectrum
        let mut mzml_bytes_reader =
            BufReader::new(std::fs::File::open("test_files/spectra_small.mzML").unwrap());
        let mut mzml_file =
            MzMlReader::read_indexed(&mut mzml_bytes_reader, None, false, false).unwrap();

        let mzml_spectrum = mzml_file
            .get_spectrum("controllerType=0 controllerNumber=1 scan=3865")
            .unwrap();

        // convert to simple spectrum
        let simple_spectrum: SimpleMsNSpectrum =
            SimpleMsNSpectrum::try_from(mzml_spectrum.clone()).unwrap();

        // check some values
        assert_eq!(simple_spectrum.ms_level, 2);
        assert_eq!(
            simple_spectrum.get_id(),
            "controllerType=0 controllerNumber=1 scan=3865"
        );
        assert_eq!(simple_spectrum.ms_level, 2);
        assert_eq!(simple_spectrum.precursors.len(), 1);
        assert_eq!(
            simple_spectrum.precursors[0].get_parent_id(),
            "controllerType=0 controllerNumber=1 scan=3864"
        );
        assert_eq!(
            simple_spectrum.precursors[0]
                .get_isolation_windows()
                .as_ref()
                .unwrap(),
            &(447.346893310547, 0.800000011921, 0.800000011921)
        );
        assert_eq!(
            simple_spectrum.precursors[0]
                .get_ions()
                .as_ref()
                .unwrap()
                .len(),
            1
        );

        assert!(!simple_spectrum.get_mz().is_empty());
        assert!(!simple_spectrum.get_intensity().is_empty());
    }
}
