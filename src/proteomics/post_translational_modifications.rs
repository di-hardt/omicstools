// internal imports
use crate::chemistry::amino_acid::AminoAcid;
use crate::proteomics::peptide::Terminus;

#[derive(PartialEq)]
pub enum Position {
    Anywhere,
    Terminus(Terminus),
}

#[derive(PartialEq)]
pub enum ModificationType {
    Static,
    Variable,
}

pub struct PostTranslationalModification {
    name: &'static str,
    amino_acid: &'static dyn AminoAcid,
    mass_delta: f64,
    total_mono_mass: f64,
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
        let total_mono_mass = amino_acid.get_mono_mass() + mass_delta;
        return Self {
            name,
            amino_acid,
            mass_delta,
            total_mono_mass,
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

    /// Returns the total mono mass
    /// This is the mass of the amino acid + the mass delta
    ///
    pub fn get_total_mono_mass(&self) -> &f64 {
        return &self.total_mono_mass;
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

    /// Returns true if the modification is static
    ///
    pub fn is_static(&self) -> bool {
        return self.mod_type == ModificationType::Static;
    }

    /// Returns true if the modification is variable
    ///
    pub fn is_variable(&self) -> bool {
        return self.mod_type == ModificationType::Variable;
    }

    /// Returns true if the modification is a terminus modification
    ///
    pub fn is_terminus(&self) -> bool {
        return self.position != Position::Anywhere; // Check if it is Anywhere otherwise we need to check for each terminus
    }

    /// Returns true if the modification is a terminus modification
    ///
    pub fn is_n_terminus(&self) -> bool {
        return self.position == Position::Terminus(Terminus::N);
    }

    /// Returns true if the modification is a terminus modification
    pub fn is_c_terminus(&self) -> bool {
        return self.position == Position::Terminus(Terminus::C);
    }

    /// Returns true if the modification is a terminus modification
    ///
    pub fn is_anywhere(&self) -> bool {
        return self.position == Position::Anywhere;
    }
}
