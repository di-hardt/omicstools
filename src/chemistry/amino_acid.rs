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
