// 3rd party imports
use anyhow::{bail, Result};

// internal imports
use crate::proteomics::proteases::protease::Protease;
use crate::proteomics::proteases::trypsin::{Trypsin, NAME as TRYPSIN_NAME};
use crate::proteomics::proteases::unspecific::{Unspecific, NAME as UNSPECIFIC_NAME};

/// Returns a protease by name
///
/// # Arguments
/// * `name` - Name of the protease
/// * `min_len` - Minimum peptide length
/// * `max_len` - Maximum peptide length
/// * `max_missed_cleavages` - Maximum number of missed cleavages (not used for unspecific proteases)
///
pub fn get_by_name(
    name: &str,
    min_len: Option<usize>,
    max_len: Option<usize>,
    max_missed_cleavages: Option<usize>,
) -> Result<Box<dyn Protease>> {
    match name.to_lowercase().as_str() {
        TRYPSIN_NAME => Ok(Box::new(Trypsin::new(
            min_len,
            max_len,
            max_missed_cleavages,
        )?)),
        UNSPECIFIC_NAME => Ok(Box::new(Unspecific::new(min_len, max_len)?)),
        _ => bail!("Protease {} not found", name),
    }
}
