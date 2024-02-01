// 3rd party imports
use anyhow::{bail, Context, Result};

/// Krokhin-Wilkins retention coefficient for amino acids
///
pub struct RetentionCoefficient {
    rn: f32,
    rcnt: f32,
}

impl RetentionCoefficient {
    pub fn get_rn(&self) -> f32 {
        self.rn
    }

    pub fn get_rcnt(&self) -> f32 {
        self.rcnt
    }
}

// Include amino acids from data/krokhin_wilkins_retention_coefficients.csv
// see build.rs
include!(concat!(
    env!("OUT_DIR"),
    "/krokhin_wilkins_hydrophobicity.rs"
));

/// Get correction coefficient (K_L)
///
/// # Arguments
/// * `sequence_len` - Length of the peptide sequence
///
pub fn get_correction_coefficient(sequence_len: u32) -> f32 {
    if sequence_len < 10 {
        1.0 - 0.027 * (10.0 - sequence_len as f32)
    } else if sequence_len > 20 {
        1.0 - 0.014 * (sequence_len as f32 - 20.0)
    } else {
        1.0
    }
}

/// Calculate hydrophobicity
///
/// # Arguments
/// * `sequence` - Amino acid sequence
///
pub fn calc_hydrophobicity(sequence: &str) -> Result<f32> {
    if sequence.len() < 3 {
        bail!("Sequence length must be at least 3");
    }

    let correction_coefficient = get_correction_coefficient(sequence.len() as u32);
    let retention_coefficient_sum = sequence
        .chars()
        .map(|code| {
            Ok(get_retention_coefficient(code)
                .context("Error when calculating hydrophobicity")?
                .get_rn())
        })
        .sum::<Result<f32>>()?;

    // first 3 retention coefficients
    let retention_coefficients_first_three = sequence
        .chars()
        .take(3)
        .map(|code| {
            Ok(get_retention_coefficient(code).context("Error when calculating hydrophobicity")?)
        })
        .collect::<Result<Vec<_>>>()?;

    let mut hydrophobicity = correction_coefficient
        * (retention_coefficient_sum
            + 0.42 * retention_coefficients_first_three[0].get_rcnt()
            + 0.22 * retention_coefficients_first_three[1].get_rcnt()
            + 0.05 * retention_coefficients_first_three[2].get_rcnt());

    if hydrophobicity >= 38.0 {
        hydrophobicity = hydrophobicity - 0.3 * (hydrophobicity - 38.0);
    }

    Ok(hydrophobicity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_krokhin_wilkins_hypophibicity() {
        // Test peptides from paper with known hydrophobicity
        let hydrophobicity = calc_hydrophobicity("SCHTAVGR").unwrap();
        // Rounding errors. Round to 2 decimal places to fix this
        let hydrophobicity = (hydrophobicity * 100.0).round() / 100.0;
        assert_eq!(hydrophobicity, 4.05);

        let hydrophobicity = calc_hydrophobicity("SASDLTWDNLK").unwrap();
        // Rounding errors. Round to 2 decimal places to fix this
        let hydrophobicity = (hydrophobicity * 100.0).round() / 100.0;
        assert_eq!(hydrophobicity, 27.72);

        // Containts Pyrrolysine, assert err
        assert!(calc_hydrophobicity("SCHTAVGRO").is_err());
        // Too short, assert err
        assert!(calc_hydrophobicity("SC").is_err());
    }
}
