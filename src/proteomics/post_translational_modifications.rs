// std imports
use std::str::FromStr;

// 3rd party imports
use anyhow::{bail, Error};

// internal imports
use crate::chemistry::amino_acid::AminoAcid;
use crate::proteomics::peptide::Terminus;

#[derive(Clone, PartialEq)]
pub enum Position {
    Anywhere,           // any residue
    Terminus(Terminus), // terminal residue
    Bond(Terminus),     // bond to another amino acid
}

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lower = s.to_lowercase();
        if s_lower == "anywhere" {
            return Ok(Position::Anywhere);
        }

        if s_lower.starts_with("terminus_") {
            let terminus_str = s_lower[9..].to_lowercase();
            let terminus = Terminus::from_str(terminus_str.as_str())?;
            return Ok(Position::Terminus(terminus));
        }

        if s_lower.starts_with("bond_") {
            let terminus_str = s_lower[9..].to_lowercase();
            let terminus = Terminus::from_str(terminus_str.as_str())?;
            return Ok(Position::Terminus(terminus));
        }

        bail!("Invalid position. Valid format: `anywhere`, `terminus_<N|C>`, `bond_<N|C>`");
    }
}

#[derive(Clone, PartialEq)]
pub enum ModificationType {
    Static,
    Variable,
}

impl FromStr for ModificationType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "static" => Ok(ModificationType::Static),
            "variable" => Ok(ModificationType::Variable),
            _ => bail!("Invalid modification type: valid types are `static` or `variable`"),
        }
    }
}

#[derive(Clone)]
pub struct PostTranslationalModification {
    name: String,
    amino_acid: &'static dyn AminoAcid,
    mass_delta: f64,
    total_mono_mass: f64,
    mod_type: ModificationType,
    position: Position,
}

impl PostTranslationalModification {
    pub fn new(
        name: &str,
        amino_acid: &'static dyn AminoAcid,
        mass_delta: f64,
        mod_type: ModificationType,
        position: Position,
    ) -> Self {
        let total_mono_mass = amino_acid.get_mono_mass() + mass_delta;
        return Self {
            amino_acid,
            mass_delta,
            total_mono_mass,
            mod_type,
            position,
            name: name.to_owned(),
        };
    }

    /// Returns the name
    ///
    pub fn get_name(&self) -> &str {
        return &self.name.as_str();
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
        return match self.position {
            Position::Terminus(_) => true,
            _ => false,
        };
    }

    /// Returns true if the modification is a terminus modification
    ///
    pub fn is_n_terminus(&self) -> bool {
        return match self.position {
            Position::Terminus(Terminus::N) => true,
            _ => false,
        };
    }

    /// Returns true if the modification is a terminus modification
    pub fn is_c_terminus(&self) -> bool {
        return match self.position {
            Position::Terminus(Terminus::C) => true,
            _ => false,
        };
    }

    /// Returns true if the modification is a bond modification
    ///
    pub fn is_bond(&self) -> bool {
        return match self.position {
            Position::Bond(_) => true,
            _ => false,
        };
    }

    /// Returns true if the modification is a N terminus bond modification
    ///
    pub fn is_n_bond(&self) -> bool {
        return match self.position {
            Position::Bond(Terminus::N) => true,
            _ => false,
        };
    }

    /// Returns true if the modification is a C terminus bond modification
    pub fn is_c_bond(&self) -> bool {
        return match self.position {
            Position::Bond(Terminus::C) => true,
            _ => false,
        };
    }

    /// Returns true if the modification is a terminus modification
    ///
    pub fn is_anywhere(&self) -> bool {
        return self.position == Position::Anywhere;
    }
}
