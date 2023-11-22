// 3rd party imports
use anyhow::Result;

// internal imports
use crate::proteomics::proteases::protease::Protease;

pub const NAME: &str = "unspecific";

pub struct Unspecific {
    min_length: Option<usize>,
    max_length: Option<usize>,
}

impl Unspecific {
    pub fn new(min_length: Option<usize>, max_length: Option<usize>) -> Result<Self> {
        Ok(Self {
            min_length,
            max_length,
        })
    }
}

impl Protease for Unspecific {
    fn get_name(&self) -> &str {
        return NAME;
    }

    fn get_min_length(&self) -> Option<usize> {
        self.min_length
    }

    fn get_max_length(&self) -> Option<usize> {
        self.max_length
    }

    fn get_max_missed_cleavages(&self) -> Option<usize> {
        // For unspecific proteases, the number of missed cleavages is not defined.
        None
    }

    fn is_count_missed_cleavages(&self) -> bool {
        // For unspecific proteases, the number of missed cleavages is not defined.
        false
    }

    fn full_digest(&self, sequence: &str) -> Result<Vec<String>> {
        Ok(sequence.chars().map(|c| c.to_string()).collect::<Vec<_>>())
    }

    fn count_missed_cleavages(&self, _sequence: &str) -> Result<usize> {
        // For unspecific proteases, the number of missed cleavages is not defined.
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    /// 3rd party imports
    use fallible_iterator::FallibleIterator;

    /// internal imports
    use super::*;
    use crate::proteomics::peptide::Peptide;

    const EXPECTED_PEPTIDES_UNLIMITED: [&'static str; 28] = [
        "P", "PE", "PEP", "PEPT", "PEPTI", "PEPTID", "PEPTIDE", "E", "EP", "EPT", "EPTI", "EPTID",
        "EPTIDE", "P", "PT", "PTI", "PTID", "PTIDE", "T", "TI", "TID", "TIDE", "I", "ID", "IDE",
        "D", "DE", "E",
    ];

    const EXPECTED_PEPTIDES_LIMITED: [&'static str; 7] =
        ["PEPT", "PEPTI", "EPTI", "EPTID", "PTID", "PTIDE", "TIDE"];

    #[test]
    fn test_cleave_unlimited() {
        let protease = Unspecific::new(None, None).unwrap();
        let sequence = "PEPTIDE".to_string();
        let peptides = protease
            .cleave(&sequence)
            .unwrap()
            .collect::<Vec<Peptide>>()
            .unwrap();
        assert_eq!(peptides.len(), EXPECTED_PEPTIDES_UNLIMITED.len());
        for peptide in peptides {
            assert!(EXPECTED_PEPTIDES_UNLIMITED.contains(&peptide.get_sequence().as_str()));
        }
    }

    #[test]
    fn test_cleave_limited() {
        let protease = Unspecific::new(Some(4), Some(5)).unwrap();
        let sequence = "PEPTIDE".to_string();
        let peptides: Vec<Peptide> = protease.cleave(&sequence).unwrap().collect().unwrap();
        assert_eq!(peptides.len(), EXPECTED_PEPTIDES_LIMITED.len());
        for peptide in peptides {
            assert!(EXPECTED_PEPTIDES_LIMITED.contains(&peptide.get_sequence().as_str()));
        }
    }
}
