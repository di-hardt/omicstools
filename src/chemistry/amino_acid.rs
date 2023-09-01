pub trait AminoAcid {
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
        return &self.composition;
    }
}

impl AminoAcid for CanonicalAminoAcid {
    fn get_name(&self) -> &'static str {
        return &self.name;
    }

    fn get_one_letter_code(&self) -> &char {
        return &self.one_letter_code;
    }

    fn get_code(&self) -> &char {
        return &self.one_letter_code;
    }

    fn get_three_letter_code(&self) -> &'static str {
        return &self.three_letter_code;
    }

    fn get_abbreviation(&self) -> &'static str {
        return &self.three_letter_code;
    }

    fn get_mono_mass(&self) -> &f64 {
        return &self.mono_mass;
    }

    fn get_average_mass(&self) -> &f64 {
        return &self.average_mass;
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
        return &self.name;
    }

    /// Returns the one letter code of the amino acid.
    ///
    fn get_one_letter_code(&self) -> &char {
        return &self.one_letter_code;
    }

    /// Synonym for `get_one_letter_code`
    ///
    fn get_code(&self) -> &char {
        return &self.one_letter_code;
    }

    /// Returns the three letter code of the amino acid.
    ///
    fn get_three_letter_code(&self) -> &'static str {
        return &self.three_letter_code;
    }

    /// Synonym for `get_three_letter_code`
    ///
    fn get_abbreviation(&self) -> &'static str {
        return &self.three_letter_code;
    }

    /// Returns the monoisotopic mass of the amino acid.
    fn get_mono_mass(&self) -> &f64 {
        return &self.mono_mass;
    }

    /// Returns the average mass of the amino acid.
    ///
    fn get_average_mass(&self) -> &f64 {
        return &self.average_mass;
    }
}

// Include amino acids from data/canonical_amino_acids.csv & data/non_canonical_amino_acids.csv
include!(concat!(env!("OUT_DIR"), "/amino_acid.rs"));

#[cfg(test)]
mod test {
    // std imports
    use std::fs::read;

    // local imports
    use super::*;

    const CANONICAL_AA_FILE: &'static str = "data/canonical_amino_acids.csv";
    const NON_CANONICAL_AA_FILE: &'static str = "data/non_canonical_amino_acids.csv";

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
