// internal imports
use crate::chemistry::amino_acid::AminoAcid;
use crate::proteomics::peptide::Terminus;

pub enum Position {
    Anywhere,
    Terminus(Terminus),
}

pub enum ModificationType {
    Static,
    Variable,
}

pub struct PostTranslationalModification {
    name: &'static str,
    amino_acid: &'static dyn AminoAcid,
    mass_delta: f64,
    mod_type: ModificationType,
    position: Position,
}

impl PostTranslationalModification {
    pub fn new(
        name: &'static str,
        amino_acid: &'static dyn AminoAcid,
        mass_delta: f64,
        mod_type: ModificationType,
        position: Position,
    ) -> Self {
        return Self {
            name,
            amino_acid,
            mass_delta,
            mod_type,
            position,
        };
    }

    /// Returns the name
    ///
    pub fn get_name(&self) -> &'static str {
        return &self.name;
    }

    /// Returns the amino acid
    ///
    pub fn get_amino_acid(&self) -> &'static dyn AminoAcid {
        return self.amino_acid;
    }

    /// Returns the mass delta
    ///
    pub fn get_mass_delta(&self) -> &f64 {
        return &self.mass_delta;
    }

    /// Returns the modification type
    ///
    pub fn get_mod_type(&self) -> &ModificationType {
        return &self.mod_type;
    }

    /// Returns the position
    ///
    pub fn get_position(&self) -> &Position {
        return &self.position;
    }
}
