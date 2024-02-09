// 3rd party imports
use anyhow::{bail, Context, Result};

/// Highly hydrophobic amino acids (one letter codes)
///
const HIGHLY_HYDROPHOBIC_AMINO_ACIDS_V3: [char; 5] = ['W', 'F', 'L', 'I', 'Y'];

/// Highly hydrophobic amino acids (one letter codes)
///
const HYDROPHOBIC_AMINO_ACIDS_V3: [char; 3] = ['M', 'V', 'A'];

/// Hydrophobic cluster weight
///
const HYDROPHOBIC_CLUSTER_WEIGHT_V3: f32 = 0.4;

/// Hydrophobic cluster characters
///
const HYDROPHOBIC_CLUSTER_CHARS_V3: [char; 2] = ['X', 'Y'];

// TODO: Find the original cluster coefficients for the hydrophobic cluster correction as well as for helical correction

/// Krokhin retention coefficient v1 for amino acids
///
pub struct RetentionCoefficientV1 {
    rn: f32,
    rcnt: f32,
}

impl RetentionCoefficientV1 {
    pub fn get_rn(&self) -> f32 {
        self.rn
    }

    pub fn get_rcnt(&self) -> f32 {
        self.rcnt
    }
}

/// Krokhin retention coefficient v3 for amino acids
///
pub struct RetentionCoefficientV3 {
    rc: f32,
    rc1: f32,
    rc2: f32,
    rcn: f32,
    rcn_1: f32,
    rcs: f32,
    rc1s: f32,
    rc2s: f32,
    rcns: f32,
    rcn_1s: f32,
}

impl RetentionCoefficientV3 {
    /// Get retention coefficient (R<sub>C</sub>)
    /// for any other amino acid
    ///
    pub fn get_rc(&self) -> f32 {
        self.rc
    }

    /// Get retention coefficient (R<sub>C1</sub>)
    /// for amino acid 1
    ///
    pub fn get_rc1(&self) -> f32 {
        self.rc1
    }

    /// Get retention coefficient (R<sub>C2</sub>)
    /// for amino acid 2
    ///
    pub fn get_rc2(&self) -> f32 {
        self.rc2
    }

    /// Get retention coefficient (R<sub>cn</sub>)
    /// for amino acid n
    ///
    pub fn get_rcn(&self) -> f32 {
        self.rcn
    }

    /// Get retention coefficient (R<sub>n-1</sub>)
    /// for amino acid n-1
    ///
    pub fn get_rcn_1(&self) -> f32 {
        self.rcn_1
    }

    /// Get retention coefficient for short peptides (< 9 amino acids) (R<sub>cs</sub>)
    /// for other amino acids
    ///
    pub fn get_rcs(&self) -> f32 {
        self.rcs
    }

    /// Get retention coefficient for short peptides (< 9 amino acids) (R<sub>C1</sub>)
    /// for amino acid 1
    ///
    pub fn get_rc1s(&self) -> f32 {
        self.rc1s
    }

    /// Get retention coefficient for short peptides (< 9 amino acids) (R<sub>C2</sub>)
    /// for amino acid 2
    ///
    pub fn get_rc2s(&self) -> f32 {
        self.rc2s
    }

    /// Get retention coefficient for short peptides (< 9 amino acids) (R<sub>Cn</sub>)
    /// for amino acid n
    ///
    pub fn get_rcns(&self) -> f32 {
        self.rcns
    }

    /// Get retention coefficient for short peptides (< 9 amino acids) (R<sub>n-1</sub>)
    /// for amino acid n-1
    ///
    pub fn get_rcn_1s(&self) -> f32 {
        self.rcn_1s
    }
}

// Include retention coefficients for amino acids
// see build.rs
include!(concat!(env!("OUT_DIR"), "/krokhin_hydrophobicity.rs"));

/// Get correction coefficient V1 (K<sub>L</sub>)
///
/// # Arguments
/// * `sequence_len` - Length of the peptide sequence
///
pub fn get_correction_coefficient_v1(sequence_len: u32) -> f32 {
    if sequence_len < 10 {
        1.0 - 0.027 * (10.0 - sequence_len as f32)
    } else if sequence_len > 20 {
        1.0 - 0.014 * (sequence_len as f32 - 20.0)
    } else {
        1.0
    }
}

/// Calculate hydrophobicity V1
///
/// # Arguments
/// * `sequence` - Amino acid sequence
///
pub fn calc_hydrophobicity_v1(sequence: &str) -> Result<f32> {
    if sequence.len() < 3 {
        bail!("Sequence length must be at least 3");
    }

    let correction_coefficient = get_correction_coefficient_v1(sequence.len() as u32);
    let retention_coefficient_sum = sequence
        .chars()
        .map(|code| {
            Ok(get_retention_coefficient_v1(code)
                .context("Error when calculating hydrophobicity")?
                .get_rn())
        })
        .sum::<Result<f32>>()?;

    // first 3 retention coefficients
    let retention_coefficients_first_three = sequence
        .chars()
        .take(3)
        .map(|code| {
            Ok(get_retention_coefficient_v1(code)
                .context("Error when calculating hydrophobicity")?)
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

/// Get correction coefficient V3 (K<sub>L</sub>)
///
/// # Arguments
/// * `sequence_len` - Length of the peptide sequence
///
pub fn get_correction_coefficient_v3(sequence_len: u32) -> f32 {
    if sequence_len < 8 {
        1.0 - 0.055 * (8.0 - sequence_len as f32)
    } else if sequence_len > 20 {
        1.0 / (1.0 - 0.027 * (sequence_len as f32 - 20.0))
    } else {
        1.0
    }
}

fn base_hydrophobicity_short_peptides_v3(sequence: &str) -> Result<f32> {
    if sequence.len() >= 9 {
        bail!("Sequence length must be less than 9");
    }

    let mut hydrophobicity: f32 = 0.0;

    // Add first and last two amino acids
    hydrophobicity += get_retention_coefficient_v3(sequence.chars().nth(0).unwrap())
        .context("Error when calculating Krokhin hydrophobicity V3")?
        .get_rc1s();
    hydrophobicity += get_retention_coefficient_v3(sequence.chars().nth(1).unwrap())
        .context("Error when calculating Krokhin hydrophobicity V3")?
        .get_rc2s();
    hydrophobicity +=
        get_retention_coefficient_v3(sequence.chars().nth(sequence.len() - 2).unwrap())
            .context("Error when calculating Krokhin hydrophobicity V3")?
            .get_rcn_1s();
    hydrophobicity +=
        get_retention_coefficient_v3(sequence.chars().nth(sequence.len() - 1).unwrap())
            .context("Error when calculating Krokhin hydrophobicity V3")?
            .get_rcns();

    // Add middle amino acids
    hydrophobicity += sequence[2..sequence.len() - 2]
        .chars()
        .map(|code| {
            Ok(get_retention_coefficient_v3(code)
                .context("Error when calculating Krokhin hydrophobicity V3")?
                .get_rcs())
        })
        .sum::<Result<f32>>()?;

    Ok(hydrophobicity)
}

fn base_hydrophobicity_long_peptides_v3(sequence: &str) -> Result<f32> {
    if sequence.len() < 9 {
        bail!("Sequence length must be at least 9");
    }

    let mut hydrophobicity: f32 = 0.0;

    // Add first and last two amino acids
    hydrophobicity += get_retention_coefficient_v3(sequence.chars().nth(0).unwrap())
        .context("Error when calculating Krokhin hydrophobicity V3")?
        .get_rc1();
    hydrophobicity += get_retention_coefficient_v3(sequence.chars().nth(1).unwrap())
        .context("Error when calculating Krokhin hydrophobicity V3")?
        .get_rc2();
    hydrophobicity +=
        get_retention_coefficient_v3(sequence.chars().nth(sequence.len() - 2).unwrap())
            .context("Error when calculating Krokhin hydrophobicity V3")?
            .get_rcn_1s();
    hydrophobicity +=
        get_retention_coefficient_v3(sequence.chars().nth(sequence.len() - 1).unwrap())
            .context("Error when calculating Krokhin hydrophobicity V3")?
            .get_rcn();

    // Add middle amino acids
    hydrophobicity += sequence[2..sequence.len() - 2]
        .chars()
        .map(|code| {
            Ok(get_retention_coefficient_v3(code)
                .context("Error when calculating Krokhin hydrophobicity V3")?
                .get_rc())
        })
        .sum::<Result<f32>>()?;

    Ok(hydrophobicity)
}

/// Proline correction coefficient V3
///
/// # Arguments
/// * `sequence` - Amino acid sequence
///
pub fn get_proline_correction_coefficient_v3(sequence: &str) -> f32 {
    if sequence.contains("PPPP") {
        return 5.0;
    } else if sequence.contains("PPP") {
        return 3.5;
    } else if sequence.contains("PP") {
        return 1.2;
    }
    0.0
}

fn get_hydrophobic_cluster_correction_v3_match_pattern(
    _clustered_sequence: &str,
    pattern: &Vec<char>,
) -> f32 {
    println!("{:?}", pattern);
    // TODO: Add pattern matching for hydrophobic clusters but we need the original coefficients
    // pattern matches * pattern weight
    0.0
}

fn get_hydrophobic_cluster_correction_v3_recusive_pattern_match(
    clustered_sequence: &str,
    pos: usize,
    start_pattern: &Vec<char>,
    mut correction_coefficient: f32,
) -> f32 {
    let mut pattern = start_pattern.clone();
    for cluster_char in HYDROPHOBIC_CLUSTER_CHARS_V3.iter() {
        pattern[pos] = *cluster_char;
        if pos < pattern.len() - 1 {
            correction_coefficient += get_hydrophobic_cluster_correction_v3_recusive_pattern_match(
                clustered_sequence,
                pos + 1,
                &pattern,
                correction_coefficient,
            );
        } else if pos == pattern.len() - 1 {
            correction_coefficient +=
                get_hydrophobic_cluster_correction_v3_match_pattern(clustered_sequence, &pattern);
        }
    }
    correction_coefficient
}

pub fn get_hydrophobic_cluster_correction_v3(sequence: &str) -> f32 {
    let mut clustered_sequence: String = sequence
        .chars()
        .map(|code| {
            if HIGHLY_HYDROPHOBIC_AMINO_ACIDS_V3.contains(&code) {
                'X'
            } else if HYDROPHOBIC_AMINO_ACIDS_V3.contains(&code) {
                'Y'
            } else {
                '_'
            }
        })
        .collect();
    clustered_sequence.insert(0, '_');
    clustered_sequence.push('_');

    let mut correction_coefficient = 0.0;

    for pattern_len in 2..=6 {
        let pattern = vec!['-'; pattern_len];
        correction_coefficient += get_hydrophobic_cluster_correction_v3_recusive_pattern_match(
            &clustered_sequence,
            0,
            &pattern,
            correction_coefficient,
        );
    }

    correction_coefficient * HYDROPHOBIC_CLUSTER_WEIGHT_V3
}

/// Calculate hydrophobicity V3
///
/// # Arguments
/// * `sequence` - Amino acid sequence
///
pub fn calc_hydrophobicity_v3(sequence: &str) -> Result<f32> {
    if sequence.len() < 4 {
        bail!("Sequence length must be at least 4");
    }

    let mut hydrophobicity = if sequence.len() < 9 {
        base_hydrophobicity_short_peptides_v3(sequence)
    } else {
        base_hydrophobicity_long_peptides_v3(sequence)
    }?;

    // (2) Nearest neighbor correction (TODO)
    // (3) Clusters of hydrophobic amino acids
    hydrophobicity -= get_hydrophobic_cluster_correction_v3(sequence);
    // (4) Role of Proline residues
    hydrophobicity -= get_proline_correction_coefficient_v3(sequence);
    // (5) Role of isoelectric point (TODO)
    // (6) Short peptides, already done in base_hydrophobicity_short_peptides_v3
    // (7) Correction coefficient
    let correction_coefficient = get_correction_coefficient_v3(sequence.len() as u32);
    hydrophobicity *= correction_coefficient;

    // (8) correction of overall hydrophobicity
    hydrophobicity = if hydrophobicity < 20.0 {
        hydrophobicity
    } else if hydrophobicity < 30.0 {
        hydrophobicity - 0.27 * (hydrophobicity - 18.0)
    } else if hydrophobicity < 40.0 {
        hydrophobicity - 0.33 * (hydrophobicity - 18.0)
    } else if hydrophobicity < 50.0 {
        hydrophobicity - 0.38 * (hydrophobicity - 18.0)
    } else {
        hydrophobicity - 0.447 * (hydrophobicity - 18.0)
    };

    // (9) Influence of Peptide Propensity To Form Helical Structures. (TODO)

    Ok(hydrophobicity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_krokhin_wilkins_hypophibicity_v1() {
        // Test peptides from paper with known hydrophobicity
        let hydrophobicity = calc_hydrophobicity_v1("SCHTAVGR").unwrap();
        // Rounding errors. Round to 2 decimal places to fix this
        let hydrophobicity = (hydrophobicity * 100.0).round() / 100.0;
        assert_eq!(hydrophobicity, 4.05);

        let hydrophobicity = calc_hydrophobicity_v1("SASDLTWDNLK").unwrap();
        // Rounding errors. Round to 2 decimal places to fix this
        let hydrophobicity = (hydrophobicity * 100.0).round() / 100.0;
        assert_eq!(hydrophobicity, 27.72);

        // Containts Pyrrolysine, assert err
        assert!(calc_hydrophobicity_v1("SCHTAVGRO").is_err());
        // Too short, assert err
        assert!(calc_hydrophobicity_v1("SC").is_err());
    }

    #[test]
    fn test_proline_correction_coeficcient() {
        let correction = get_proline_correction_coefficient_v3("GGLSIISPPPPEK");
        assert_eq!(correction, 5.0);
        let correction = get_proline_correction_coefficient_v3("GGLSIISPPPEK");
        assert_eq!(correction, 3.5);
        let correction = get_proline_correction_coefficient_v3("GGLSIISPPEK");
        assert_eq!(correction, 1.2);
    }

    #[test]
    fn test_krokhin_wilkins_hypophibicity_v3() {
        // Test peptides from paper with known hydrophobicity
        let mut hydrophobicity = calc_hydrophobicity_v3("LVEYR").unwrap();
        // Rounding errors. Round to 2 decimal places to fix this
        hydrophobicity = (hydrophobicity * 100.0).round() / 100.0;
        // TODO: Add remaining correction coefficients and uncomment tests below
        // assert_eq!(hydrophobicity, 10.69);

        // // Test peptides from paper with known hydrophobicity
        // let hydrophobicity = calc_hydrophobicity_v1("GGLSIISPPEK").unwrap();
        // // Rounding errors. Round to 2 decimal places to fix this
        // let hydrophobicity = (hydrophobicity * 100.0).round() / 100.0;
        // assert_eq!(hydrophobicity, 23.33);

        // let hydrophobicity = calc_hydrophobicity_v1("SASDLTWDNLK").unwrap();
        // // Rounding errors. Round to 2 decimal places to fix this
        // let hydrophobicity = (hydrophobicity * 100.0).round() / 100.0;
        // assert_eq!(hydrophobicity, 27.72);

        // // Containts Pyrrolysine, assert err
        // assert!(calc_hydrophobicity_v1("SCHTAVGRO").is_err());
        // // Too short, assert err
        // assert!(calc_hydrophobicity_v1("SC").is_err());
    }
}
