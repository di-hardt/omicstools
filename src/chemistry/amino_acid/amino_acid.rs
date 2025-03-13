// Sync and Send are required for the amino acids to be used in parallel
// and should be save as the amino acids are immutable.
pub trait AminoAcid: Sync + Send {
    /// Returns the name of the amino acid.
    ///
    fn get_name(&self) -> &'static str;

    /// Returns the one letter code of the amino acid.
    ///
    fn get_one_letter_code(&self) -> &char;

    /// Returns the one letter code of the amino acid.
    /// Alias for `get_one_letter_code`
    ///
    fn get_code(&self) -> &char;

    /// Returns the three letter code of the amino acid.
    ///
    fn get_three_letter_code(&self) -> &'static str;

    /// Returns the three letter code of the amino acid.
    /// Alias for `get_three_letter_code`
    ///
    fn get_abbreviation(&self) -> &'static str;

    /// Returns the monoisotopic mass of the amino acid.
    ///
    fn get_mono_mass(&self) -> &f64;

    /// Returns the average mass of the amino acid.
    ///
    fn get_average_mass(&self) -> &f64;
}

impl serde::Serialize for &dyn AminoAcid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.get_one_letter_code().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for &dyn AminoAcid {
    fn deserialize<D>(deserializer: D) -> Result<&'static dyn AminoAcid, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let code = char::deserialize(deserializer)?;
        match get_amino_acid_by_one_letter_code(code) {
            Ok(amino_acid) => Ok(amino_acid),
            Err(_) => Err(serde::de::Error::custom(format!(
                "Unknown amino acid code: {}",
                code
            ))),
        }
    }
}

/// Contains various information about an amino acid.
///
pub struct CanonicalAminoAcid {
    name: &'static str,
    one_letter_code: char,
    three_letter_code: &'static str,
    composition: &'static str,
    mono_mass: f64,
    average_mass: f64,
}

impl CanonicalAminoAcid {
    /// Returns the composition of the amino acid.
    ///
    pub fn get_composition(&self) -> &'static str {
        self.composition
    }
}

impl AminoAcid for CanonicalAminoAcid {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_one_letter_code(&self) -> &char {
        &self.one_letter_code
    }

    fn get_code(&self) -> &char {
        &self.one_letter_code
    }

    fn get_three_letter_code(&self) -> &'static str {
        self.three_letter_code
    }

    fn get_abbreviation(&self) -> &'static str {
        self.three_letter_code
    }

    fn get_mono_mass(&self) -> &f64 {
        &self.mono_mass
    }

    fn get_average_mass(&self) -> &f64 {
        &self.average_mass
    }
}

/// Contains various information about an amino acid.
///
pub struct NonCanonicalAminoAcid {
    name: &'static str,
    one_letter_code: char,
    three_letter_code: &'static str,
    mono_mass: f64,
    average_mass: f64,
}

impl AminoAcid for NonCanonicalAminoAcid {
    /// Returns the name of the amino acid.
    ///
    fn get_name(&self) -> &'static str {
        self.name
    }

    /// Returns the one letter code of the amino acid.
    ///
    fn get_one_letter_code(&self) -> &char {
        &self.one_letter_code
    }

    /// Synonym for `get_one_letter_code`
    ///
    fn get_code(&self) -> &char {
        &self.one_letter_code
    }

    /// Returns the three letter code of the amino acid.
    ///
    fn get_three_letter_code(&self) -> &'static str {
        self.three_letter_code
    }

    /// Synonym for `get_three_letter_code`
    ///
    fn get_abbreviation(&self) -> &'static str {
        self.three_letter_code
    }

    /// Returns the monoisotopic mass of the amino acid.
    fn get_mono_mass(&self) -> &f64 {
        &self.mono_mass
    }

    /// Returns the average mass of the amino acid.
    ///
    fn get_average_mass(&self) -> &f64 {
        &self.average_mass
    }
}

impl serde::Serialize for NonCanonicalAminoAcid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.one_letter_code.serialize(serializer)
    }
}

// Include amino acids from data/canonical_amino_acids.csv & data/non_canonical_amino_acids.csv
include!(concat!(env!("OUT_DIR"), "/amino_acid.rs"));

/// Get amino acid hydropathicity according to Kyte & Doolittle (https://doi.org/10.1016/0022-2836(82)90515-0.)
///
/// # Arguments
/// * `code` - One letter code of the amino acid
///
pub fn get_hydropathicity_kd(code: char) -> Result<f64> {
    match code {
        'G' => Ok(-0.4),
        'A' => Ok(1.8),
        'S' => Ok(-0.8),
        'P' => Ok(-1.6),
        'V' => Ok(4.2),
        'T' => Ok(-0.7),
        'C' => Ok(2.5),
        'L' => Ok(3.8),
        'I' => Ok(4.5),
        'N' => Ok(-3.5),
        'D' => Ok(-3.5),
        'Q' => Ok(-3.5),
        'K' => Ok(-3.9),
        'E' => Ok(-3.5),
        'M' => Ok(1.9),
        'H' => Ok(-3.2),
        'F' => Ok(2.8),
        'R' => Ok(-4.5),
        'Y' => Ok(-1.3),
        'W' => Ok(-0.9),
        _ => bail!("Unknown amino acid code: {}", code),
    }
}

// /// Deserializes amino acid from one letter code.
// ///
// pub fn deserialize_amino_acid_from_code<'de, D>(
//     deserializer: D,
// ) -> Result<&'de dyn AminoAcid, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let code = char::deserialize(deserializer)?;
//     match get_amino_acid_by_one_letter_code(code) {
//         Ok(amino_acid) => Ok(amino_acid),
//         Err(_) => Err(serde::de::Error::custom(format!(
//             "Unknown amino acid code: {}",
//             code
//         ))),
//     }
// }

#[cfg(test)]
mod test {
    // std imports
    use std::fs::read;

    // local imports
    use super::*;

    const CANONICAL_AA_FILE: &str = "data/canonical_amino_acids.csv";
    const NON_CANONICAL_AA_FILE: &str = "data/non_canonical_amino_acids.csv";

    #[test]
    fn test_completeness() {
        let plain_canonical_amino_acids =
            String::from_utf8(read(CANONICAL_AA_FILE).unwrap()).unwrap();
        let plain_non_canonical_amino_acids =
            String::from_utf8(read(NON_CANONICAL_AA_FILE).unwrap()).unwrap();

        let canonical_amino_acids_lines: Vec<String> = plain_canonical_amino_acids
            .split("\n")
            .map(|line| line.to_owned())
            .collect();
        let non_canonical_amino_acids_lines: Vec<String> = plain_non_canonical_amino_acids
            .split("\n")
            .map(|line| line.to_owned())
            .collect();

        // -1 because of the headers
        assert!(CANONICAL_AMINO_ACIDS.len() == canonical_amino_acids_lines.len() - 1);

        // -1 because of the headers
        assert!(NON_CANONICAL_AMINO_ACIDS.len() == non_canonical_amino_acids_lines.len() - 1);

        for amino_acid_lines in [canonical_amino_acids_lines, non_canonical_amino_acids_lines] {
            for line in &amino_acid_lines[1..] {
                let attributes: Vec<&str> = line.split(",").collect();
                let one_letter_code = attributes[1].chars().next().unwrap();
                let amino_acid = get_amino_acid_by_one_letter_code(one_letter_code).unwrap();
                assert!(*amino_acid.get_code() == one_letter_code)
            }
        }
    }

    #[test]
    fn test_getting_unkown_amino_acid() {
        assert!(get_amino_acid_by_one_letter_code('Ä').is_err());
    }
}
