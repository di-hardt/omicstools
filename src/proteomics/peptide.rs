// std imports
use std::str::FromStr;
use std::string::ToString;

// 3rd party imports
use anyhow::{bail, Context, Error, Result};

// internal imports
use crate::chemistry::amino_acid::get_amino_acid_by_one_letter_code;
use crate::chemistry::element::get_element_by_symbol;

lazy_static! {
    /// Monoisotopic mass of water
    /// TODO: If there is any plans to add a molecule module, this constant should be moved there.
    ///
    static ref WATER_MASS: f64 = get_element_by_symbol("H").unwrap().get_mono_mass() * 2.0
        + get_element_by_symbol("O").unwrap().get_mono_mass();
}

#[derive(Clone, Debug, PartialEq)]
pub enum Terminus {
    N,
    C,
}

impl FromStr for Terminus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "N" => Ok(Terminus::N),
            "C" => Ok(Terminus::C),
            _ => bail!("Invalid terminus. Valid termini are `N` or `C`"),
        }
    }
}

impl ToString for Terminus {
    fn to_string(&self) -> String {
        match self {
            Self::N => "N".to_owned(),
            Self::C => "C".to_owned(),
        }
    }
}

/// Calculates the mass of a peptide sequence
/// plus the mass of water which is lost due to the peptide bond formation.
///
/// # Arguments
/// * `sequence` - A peptide sequence
///
pub fn calculate_mass_of_peptide_sequence(sequence: &str) -> Result<f64> {
    Ok(*WATER_MASS
        + sequence
            .chars()
            .map(|code| {
                Ok(get_amino_acid_by_one_letter_code(code)
                    .context("Error when calculate mass of peptide sequence")?
                    .get_mono_mass())
            })
            .sum::<Result<f64>>()?)
}

/// Very simple peptide representation to start with
///
#[derive(Clone, Debug, PartialEq)]
pub struct Peptide {
    sequence: String,
    missed_cleavages: usize,
    mass: f64,
}

impl Peptide {
    pub fn new(sequence: String, missed_cleavages: usize) -> Result<Self> {
        let mass = calculate_mass_of_peptide_sequence(&sequence)?;
        Ok(Self {
            sequence,
            missed_cleavages,
            mass,
        })
    }

    /// Returns the sequence
    ///
    pub fn get_sequence(&self) -> &String {
        &self.sequence
    }

    /// Returns the number of missed cleavages
    ///
    pub fn get_missed_cleavages(&self) -> usize {
        self.missed_cleavages
    }

    /// Returns the mass
    ///
    pub fn get_mass(&self) -> f64 {
        self.mass
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminus_from_str() {
        assert_eq!(Terminus::from_str("N").unwrap(), Terminus::N);
        assert_eq!(Terminus::from_str("C").unwrap(), Terminus::C);
        assert!(Terminus::from_str("X").is_err());
    }

    #[test]
    fn test_terminus_to_string() {
        assert_eq!(Terminus::N.to_string(), "N");
        assert_eq!(Terminus::C.to_string(), "C");
    }

    #[test]
    fn test_calculate_mass_of_peptide_sequence() {
        let mut mass =
            calculate_mass_of_peptide_sequence("VEYLDDRNTFRHSVVVPYEPPEVGSDCTTIHYNYMCNSSCMGGMNR")
                .unwrap();
        // As we have a couple decimal places in place, there are some rounding errors.
        // the returned mass is actually: `5285.286805615001`
        // So we round it again 9 decimal places.
        mass = (mass * 1000000000.0).round() / 1000000000.0;

        assert_eq!(mass, 5285.286805615);
    }
}
