use std::fmt::Display;
use std::str::FromStr;
use std::string::ToString;

use anyhow::{bail, Error};

use crate::chemistry::amino_acid::AminoAcid;
use crate::proteomics::peptide::Terminus;

#[derive(Clone, Debug, PartialEq)]
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

        if s_lower.starts_with("terminus-") {
            let terminus_str = s_lower.strip_prefix("terminus-").unwrap_or_default();
            let terminus = Terminus::from_str(terminus_str)?;
            return Ok(Position::Terminus(terminus));
        }

        if s_lower.starts_with("bond-") {
            let terminus_str = s_lower.strip_prefix("bond-").unwrap_or_default();
            let terminus = Terminus::from_str(terminus_str)?;
            return Ok(Position::Bond(terminus));
        }

        bail!("Invalid position. Valid format: `Anywhere`, `Terminus-<N|C>`, `Bond-<N|C>`");
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anywhere => write!(f, "Anywhere"),
            Self::Terminus(terminus) => write!(f, "Terminus-{}", terminus),
            Self::Bond(terminus) => write!(f, "Bond-{}", terminus),
        }
    }
}

impl serde::Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Position, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Position::from_str(s.as_str()).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
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
            _ => bail!("Invalid modification type: valid types are `Static` or `Variable`"),
        }
    }
}

impl Display for ModificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static => write!(f, "Static"),
            Self::Variable => write!(f, "Variable"),
        }
    }
}

// Deserialization is implemented manually to calculate the total mono mass on the fly
#[derive(Clone, serde::Serialize)]
pub struct PostTranslationalModification {
    name: String,
    amino_acid: &'static dyn AminoAcid,
    mass_delta: f64,
    #[serde(skip_serializing)]
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
        Self {
            amino_acid,
            mass_delta,
            total_mono_mass,
            mod_type,
            position,
            name: name.to_owned(),
        }
    }

    /// Returns the name
    ///
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the amino acid
    ///
    pub fn get_amino_acid(&self) -> &'static dyn AminoAcid {
        self.amino_acid
    }

    /// Returns the mass delta
    ///
    pub fn get_mass_delta(&self) -> &f64 {
        &self.mass_delta
    }

    /// Returns the total mono mass
    /// This is the mass of the amino acid + the mass delta
    ///
    pub fn get_total_mono_mass(&self) -> &f64 {
        &self.total_mono_mass
    }

    /// Returns the modification type
    ///
    pub fn get_mod_type(&self) -> &ModificationType {
        &self.mod_type
    }

    /// Returns the position
    ///
    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// Returns true if the modification is static
    ///
    pub fn is_static(&self) -> bool {
        self.mod_type == ModificationType::Static
    }

    /// Returns true if the modification is variable
    ///
    pub fn is_variable(&self) -> bool {
        self.mod_type == ModificationType::Variable
    }

    /// Returns true if the modification is a terminus modification
    ///
    pub fn is_terminus(&self) -> bool {
        matches!(self.position, Position::Terminus(_))
    }

    /// Returns true if the modification is a terminus modification
    ///
    pub fn is_n_terminus(&self) -> bool {
        self.position == Position::Terminus(Terminus::N)
    }

    /// Returns true if the modification is a terminus modification
    pub fn is_c_terminus(&self) -> bool {
        self.position == Position::Terminus(Terminus::C)
    }

    /// Returns true if the modification is a bond modification
    ///
    pub fn is_bond(&self) -> bool {
        matches!(self.position, Position::Bond(_))
    }

    /// Returns true if the modification is a N terminus bond modification
    ///
    pub fn is_n_bond(&self) -> bool {
        self.position == Position::Bond(Terminus::N)
    }

    /// Returns true if the modification is a C terminus bond modification
    pub fn is_c_bond(&self) -> bool {
        self.position == Position::Bond(Terminus::C)
    }

    /// Returns true if the modification is a terminus modification
    ///
    pub fn is_anywhere(&self) -> bool {
        self.position == Position::Anywhere
    }
}

/// Helper to deserialize the PostTranslationalModification without
/// without the total mono mass
/// which is calculated when the struct is converted using `into()?s
///
#[derive(serde::Deserialize)]
struct PostTranslationalModificationFields {
    name: String,
    amino_acid: &'static dyn AminoAcid,
    mass_delta: f64,
    mod_type: ModificationType,
    position: Position,
}

// Convert the helper into a PostTranslationalModification
#[allow(clippy::from_over_into)]
impl Into<PostTranslationalModification> for PostTranslationalModificationFields {
    fn into(self) -> PostTranslationalModification {
        PostTranslationalModification {
            name: self.name,
            amino_acid: self.amino_acid,
            mass_delta: self.mass_delta,
            mod_type: self.mod_type,
            position: self.position,
            total_mono_mass: self.amino_acid.get_mono_mass() + self.mass_delta,
        }
    }
}

// Manual deserialization to calculate total mono mass after deserialize
impl<'de> serde::Deserialize<'de> for PostTranslationalModification {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let fields = <PostTranslationalModificationFields>::deserialize(deserializer)?;
        Ok(fields.into())
    }
}

// Making the struct Send + Sync should be save as it is a read-only struct
unsafe impl Send for PostTranslationalModification {}

#[cfg(test)]
mod tests {
    // std imports
    use std::env;
    use std::fs::read_to_string;
    use std::fs::remove_file;
    use std::path::Path;

    use crate::chemistry::amino_acid::get_amino_acid_by_one_letter_code;

    // internal imports
    use super::*;

    const EXPECTED_AMINO_ACIDS: [(&str, char, f64, ModificationType, Position, f64); 5] = [
        (
            "Mod0",
            'C',
            57.021464,
            ModificationType::Static,
            Position::Anywhere,
            103.009184505 + 57.021464,
        ),
        (
            "Mod1",
            'M',
            15.994915,
            ModificationType::Variable,
            Position::Terminus(Terminus::N),
            131.040484645 + 15.994915,
        ),
        (
            "Mod2",
            'R',
            47.123,
            ModificationType::Static,
            Position::Terminus(Terminus::C),
            156.101111050 + 47.123,
        ),
        (
            "Mod3",
            'K',
            123.456,
            ModificationType::Variable,
            Position::Bond(Terminus::N),
            128.094963050 + 123.456,
        ),
        (
            "Mod4",
            'D',
            185.90,
            ModificationType::Static,
            Position::Bond(Terminus::C),
            115.026943065 + 185.90,
        ),
    ];

    #[test]
    fn test_position_from_str() {
        assert_eq!(Position::from_str("anywhere").unwrap(), Position::Anywhere);
        assert_eq!(
            Position::from_str("Terminus-N").unwrap(),
            Position::Terminus(Terminus::N)
        );
        assert_eq!(
            Position::from_str("Terminus-C").unwrap(),
            Position::Terminus(Terminus::C)
        );
        assert_eq!(
            Position::from_str("Bond-N").unwrap(),
            Position::Bond(Terminus::N)
        );
        assert_eq!(
            Position::from_str("Bond-C").unwrap(),
            Position::Bond(Terminus::C)
        );
        assert!(Position::from_str("X").is_err());
    }

    #[test]
    fn test_type_from_str() {
        assert_eq!(
            ModificationType::from_str("Static").unwrap(),
            ModificationType::Static
        );
        assert_eq!(
            ModificationType::from_str("Variable").unwrap(),
            ModificationType::Variable
        );
        assert!(ModificationType::from_str("X").is_err());
    }

    #[test]
    fn test_deserialization() {
        let ptm_file_path = Path::new("test_files/ptm.csv");
        let csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_path(ptm_file_path)
            .unwrap();

        for (ptm_index, ptm) in csv_reader.into_deserialize().enumerate() {
            let ptm: PostTranslationalModification = ptm.unwrap();
            let expected_data = &EXPECTED_AMINO_ACIDS[ptm_index];
            assert_eq!(ptm.get_name(), expected_data.0);
            assert_eq!(*ptm.get_amino_acid().get_code(), expected_data.1);
            assert_eq!(*ptm.get_mass_delta(), expected_data.2);
            assert_eq!(*ptm.get_mod_type(), expected_data.3);
            assert_eq!(*ptm.get_position(), expected_data.4);
            assert_eq!(*ptm.get_total_mono_mass(), expected_data.5);
        }
    }

    #[test]
    fn test_serialization() {
        let ptms: Vec<PostTranslationalModification> = EXPECTED_AMINO_ACIDS
            .iter()
            .map(|data| {
                PostTranslationalModification::new(
                    data.0,
                    get_amino_acid_by_one_letter_code(data.1).unwrap(),
                    data.2,
                    data.3.clone(),
                    data.4.clone(),
                )
            })
            .collect();

        let tmp_path = env::temp_dir();

        let tmp_ptm_file_path = tmp_path.join("dihardts_omicstools_ptm.csv");

        let mut writer = csv::WriterBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_path(&tmp_ptm_file_path)
            .unwrap();

        for ptm in ptms.iter() {
            writer.serialize(ptm).unwrap();
        }

        writer.flush().unwrap();

        let tmp_ptm_file_content = std::fs::read_to_string(&tmp_ptm_file_path).unwrap();
        let expected_ptm_file_content = read_to_string("test_files/ptm.csv").unwrap();
        assert_eq!(tmp_ptm_file_content, expected_ptm_file_content);

        remove_file(&tmp_ptm_file_path).unwrap();
    }
}
