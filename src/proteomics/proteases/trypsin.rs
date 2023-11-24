// 3rd party imports
use anyhow::{bail, Result};
use fancy_regex::Regex;

use crate::chemistry::amino_acid::{AminoAcid, ARGININE, LYSINE, PROLINE};
// internal imports
use crate::proteomics::proteases::protease::Protease;
use crate::tools::fancy_regex::split as regex_split;

/// Protease name
///
pub const NAME: &str = "trypsin";

/// Cleavage amino acids
///
const CLEAVAGE_AMINO_ACIDS: [&'static dyn AminoAcid; 2] = [&ARGININE, &LYSINE];

/// Blocking amino acids
///
const CLEAVAGE_BLOCKING_AMINO_ACIDS: [&'static dyn AminoAcid; 1] = [&PROLINE];

lazy_static! {
    /// Regex to find cleavage sites
    ///
    static ref CLEAVAGE_SITE_REGEX: Regex = Regex::new("(?<=[KR])(?!P)").unwrap();
    static ref CLEAVAGE_AMINO_ACIDS_CHARS: [char; 2] = [*ARGININE.get_code(), *LYSINE.get_code()];
}

/// Trypsin protease, cleaves after K and R, but not if followed by P
///
pub struct Trypsin {
    min_length: Option<usize>,
    max_length: Option<usize>,
    max_missed_cleavages: Option<usize>,
}

impl Trypsin {
    /// Creates a new Trypsin instance
    ///
    pub fn new(
        min_length: Option<usize>,
        max_length: Option<usize>,
        max_missed_cleavages: Option<usize>,
    ) -> Result<Self> {
        Ok(Self {
            min_length,
            max_length,
            max_missed_cleavages,
        })
    }
}

impl Protease for Trypsin {
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
        self.max_missed_cleavages
    }

    fn is_count_missed_cleavages(&self) -> bool {
        true
    }

    fn get_cleavage_amino_acids(&self) -> &[&dyn AminoAcid] {
        &CLEAVAGE_AMINO_ACIDS
    }

    fn get_cleavage_blocking_amino_acids(&self) -> &[&dyn AminoAcid] {
        &CLEAVAGE_BLOCKING_AMINO_ACIDS
    }

    fn is_blocking_amino_acid_before_cleavage_site(&self) -> bool {
        false
    }

    fn full_digest(&self, sequence: &str) -> Result<Vec<String>> {
        Ok(regex_split(&CLEAVAGE_SITE_REGEX, sequence)?
            .iter()
            .map(|s| s.to_string())
            .collect())
    }

    fn count_missed_cleavages(&self, sequence: &str) -> Result<usize> {
        if sequence.len() == 0 {
            bail!("Empty sequence");
        }
        let mut missed_cleavages = CLEAVAGE_SITE_REGEX.find_iter(sequence).count();
        // As the cleavage site regex is also counting the last amino acid, we have to subtract one
        // if the last amino acid is a cleavage site.
        if CLEAVAGE_AMINO_ACIDS_CHARS.contains(&sequence.chars().last().unwrap()) {
            missed_cleavages -= 1;
        }
        Ok(missed_cleavages)
    }
}

#[cfg(test)]
mod test {
    // std imports
    use std::collections::HashMap;

    // 3rd party imports
    use fallible_iterator::FallibleIterator;

    // internal imports
    use super::*;
    use crate::proteomics::peptide::Peptide;

    lazy_static! {
        // Peptides for Leptin (UniProt accession Q257X2, with KP on first position) digested with 3 missed cleavages, length 0 - 60
        // Tested with https://web.expasy.org/peptide_mass/
        // Leucine and Isoleucine are replaced with J already!
        static ref EXPECTED_PEPTIDES_UNLIMITED: HashMap<String, usize> = collection! {
            "VTGLDFIPGLHPLLSLSKMDQTLAIYQQILASLPSRNVIQISNDLENLRDLLHLLAASK".to_string() => 3,
            "NVIQISNDLENLRDLLHLLAASKSCPLPQVRALESLESLGVVLEASLYSTEVVALSR".to_string() => 3,
            "DLLHLLAASKSCPLPQVRALESLESLGVVLEASLYSTEVVALSRLQGSLQDMLR".to_string() => 3,
            "QRVTGLDFIPGLHPLLSLSKMDQTLAIYQQILASLPSRNVIQISNDLENLR".to_string() => 3,
            "INDISHTQSVSSKQRVTGLDFIPGLHPLLSLSKMDQTLAIYQQILASLPSR".to_string() => 3,
            "SCPLPQVRALESLESLGVVLEASLYSTEVVALSRLQGSLQDMLRQLDLSPGC".to_string() => 3,
            "MDQTLAIYQQILASLPSRNVIQISNDLENLRDLLHLLAASKSCPLPQVR".to_string() => 3,
            "VTGLDFIPGLHPLLSLSKMDQTLAIYQQILASLPSRNVIQISNDLENLR".to_string() => 2,
            "SCPLPQVRALESLESLGVVLEASLYSTEVVALSRLQGSLQDMLR".to_string() => 2,
            "ALESLESLGVVLEASLYSTEVVALSRLQGSLQDMLRQLDLSPGC".to_string() => 2,
            "DLLHLLAASKSCPLPQVRALESLESLGVVLEASLYSTEVVALSR".to_string() => 2,
            "MDQTLAIYQQILASLPSRNVIQISNDLENLRDLLHLLAASK".to_string() => 2,
            "QRVTGLDFIPGLHPLLSLSKMDQTLAIYQQILASLPSR".to_string() => 2,
            "TIVTRINDISHTQSVSSKQRVTGLDFIPGLHPLLSLSK".to_string() => 3,
            "VTGLDFIPGLHPLLSLSKMDQTLAIYQQILASLPSR".to_string() => 1,
            "ALESLESLGVVLEASLYSTEVVALSRLQGSLQDMLR".to_string() => 1,
            "CGPLYRFLWLWPYLSYVEAVPIRKVQDDTK".to_string() => 3,
            "SCPLPQVRALESLESLGVVLEASLYSTEVVALSR".to_string() => 1,
            "INDISHTQSVSSKQRVTGLDFIPGLHPLLSLSK".to_string() => 2,
            "MDQTLAIYQQILASLPSRNVIQISNDLENLR".to_string() => 1,
            "NVIQISNDLENLRDLLHLLAASKSCPLPQVR".to_string() => 2,
            "FLWLWPYLSYVEAVPIRKVQDDTKTLIK".to_string() => 3,
            "KPMRCGPLYRFLWLWPYLSYVEAVPIRK".to_string() => 3,
            "KPMRCGPLYRFLWLWPYLSYVEAVPIR".to_string() => 2,
            "VQDDTKTLIKTIVTRINDISHTQSVSSK".to_string() => 3,
            "CGPLYRFLWLWPYLSYVEAVPIRK".to_string() => 2,
            "FLWLWPYLSYVEAVPIRKVQDDTK".to_string() => 2,
            "CGPLYRFLWLWPYLSYVEAVPIR".to_string() => 1,
            "ALESLESLGVVLEASLYSTEVVALSR".to_string() => 0,
            "TLIKTIVTRINDISHTQSVSSKQR".to_string() => 3,
            "NVIQISNDLENLRDLLHLLAASK".to_string() => 1,
            "TLIKTIVTRINDISHTQSVSSK".to_string() => 2,
            "FLWLWPYLSYVEAVPIRK".to_string() => 1,
            "TIVTRINDISHTQSVSSKQR".to_string() => 2,
            "QRVTGLDFIPGLHPLLSLSK".to_string() => 1,
            "FLWLWPYLSYVEAVPIR".to_string() => 0,
            "MDQTLAIYQQILASLPSR".to_string() => 0,
            "TIVTRINDISHTQSVSSK".to_string() => 1,
            "LQGSLQDMLRQLDLSPGC".to_string() => 1,
            "DLLHLLAASKSCPLPQVR".to_string() => 1,
            "VTGLDFIPGLHPLLSLSK".to_string() => 0,
            "KVQDDTKTLIKTIVTR".to_string() => 3,
            "VQDDTKTLIKTIVTR".to_string() => 2,
            "INDISHTQSVSSKQR".to_string() => 1,
            "NVIQISNDLENLR".to_string() => 0,
            "INDISHTQSVSSK".to_string() => 0,
            "KVQDDTKTLIK".to_string() => 2,
            "VQDDTKTLIK".to_string() => 1,
            "LQGSLQDMLR".to_string() => 0,
            "DLLHLLAASK".to_string() => 0,
            "TLIKTIVTR".to_string() => 1,
            "KPMRCGPLYR".to_string() => 1,
            "SCPLPQVR".to_string() => 0,
            "KVQDDTK".to_string() => 1,
            "QLDLSPGC".to_string() => 0,
            "CGPLYR".to_string() => 0,
            "VQDDTK".to_string() => 0,
            "TIVTR".to_string() => 0,
            "TLIK".to_string() => 0,
            "KPMR".to_string() => 0,
            "QR".to_string() => 0,
            "K".to_string() => 0
        };

        static ref EXPECTED_PEPTIDES_LIMITED: HashMap<String, usize> = EXPECTED_PEPTIDES_UNLIMITED.iter().filter_map(|(key, value)| {
            if key.len() >= 6 && key.len() <= 50 {
                Some((key.clone(), *value))
            } else {
                None
            }
        }).collect();
    }

    #[test]
    fn test_cleave_unlimited() {
        // Using Leptin (UniProt  accession: Q257X2) with KP on first position to make sure Trypin-implementation skips it
        let leptin: &'static str = "KPMRCGPLYRFLWLWPYLSYVEAVPIRKVQDDTKTLIKTIVTRINDISHTQSVSSKQRVTGLDFIPGLHPLLSLSKMDQTLAIYQQILASLPSRNVIQISNDLENLRDLLHLLAASKSCPLPQVRALESLESLGVVLEASLYSTEVVALSRLQGSLQDMLRQLDLSPGC";
        let trypsin: Trypsin = Trypsin::new(Some(0), Some(60), Some(3)).unwrap();
        let peptides: Vec<Peptide> = trypsin.cleave(leptin).unwrap().collect().unwrap();

        assert_eq!(EXPECTED_PEPTIDES_UNLIMITED.len(), peptides.len());

        for peptide in peptides.iter() {
            assert!(EXPECTED_PEPTIDES_UNLIMITED.contains_key(peptide.get_sequence()),);
            assert_eq!(
                EXPECTED_PEPTIDES_UNLIMITED[peptide.get_sequence()],
                peptide.get_missed_cleavages()
            );
        }
    }

    #[test]
    fn test_count_missed_cleavage() {
        // Using Leptin (UniProt  accession: Q257X2) with KP on first position to make sure Trypin-implementation skips it
        let leptin: &'static str = "KPMRCGPLYRFLWLWPYLSYVEAVPIRKVQDDTKTLIKTIVTRINDISHTQSVSSKQRVTGLDFIPGLHPLLSLSKMDQTLAIYQQILASLPSRNVIQISNDLENLRDLLHLLAASKSCPLPQVRALESLESLGVVLEASLYSTEVVALSRLQGSLQDMLRQLDLSPGC";
        let trypsin: Trypsin = Trypsin::new(Some(0), Some(60), Some(3)).unwrap();
        let peptides: Vec<Peptide> = trypsin.cleave(leptin).unwrap().collect().unwrap();

        for peptide in peptides.iter() {
            assert_eq!(
                trypsin
                    .count_missed_cleavages(peptide.get_sequence())
                    .unwrap(),
                EXPECTED_PEPTIDES_UNLIMITED[peptide.get_sequence()]
            );
        }
    }

    #[test]
    fn test_cleave_limited() {
        // Using Leptin (UniProt  accession: Q257X2) with KP on first position to make sure Trypin-implementation skips it
        let leptin: &'static str = "KPMRCGPLYRFLWLWPYLSYVEAVPIRKVQDDTKTLIKTIVTRINDISHTQSVSSKQRVTGLDFIPGLHPLLSLSKMDQTLAIYQQILASLPSRNVIQISNDLENLRDLLHLLAASKSCPLPQVRALESLESLGVVLEASLYSTEVVALSRLQGSLQDMLRQLDLSPGC";
        let trypsin: Trypsin = Trypsin::new(Some(6), Some(50), Some(3)).unwrap();
        let peptides: Vec<Peptide> = trypsin.cleave(leptin).unwrap().collect().unwrap();

        assert_eq!(EXPECTED_PEPTIDES_LIMITED.len(), peptides.len());

        for peptide in peptides.iter() {
            assert!(EXPECTED_PEPTIDES_LIMITED.contains_key(peptide.get_sequence()),);
            assert_eq!(
                EXPECTED_PEPTIDES_LIMITED[peptide.get_sequence()],
                peptide.get_missed_cleavages()
            );
        }
    }
}
