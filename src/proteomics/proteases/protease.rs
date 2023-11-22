// std imports
use std::cmp;

// 3rd party imports
use anyhow::{Error, Result};
use fallible_iterator::FallibleIterator;

// internal imports
use crate::proteomics::peptide::Peptide;

/// Trait defining the behavior for a protease
///
pub trait Protease: Send + Sync {
    /// Returns the name of the enzyme
    fn get_name(&self) -> &str;

    /// Returns the min peptide length
    ///
    fn get_min_length(&self) -> Option<usize>;

    /// Returns the max peptide length
    ///
    fn get_max_length(&self) -> Option<usize>;

    /// Returns the max number of missed cleavages
    ///
    fn get_max_missed_cleavages(&self) -> Option<usize>;

    /// Returns if missed cleavages are counted
    /// For unspecific proteases, this should return false
    /// (the returned peptides will have 0 missed cleavages).
    /// For specific proteases, this should return true
    /// (the returned peptides will have 0, 1, 2, ... missed cleavages).
    ///
    fn is_count_missed_cleavages(&self) -> bool;

    /// Returns the sequence digested with zero missed cleavages
    ///
    /// # Arguments
    /// * `sequence` - Amino acid sequence
    ///
    fn full_digest(&self, sequence: &str) -> Result<Vec<String>>;

    /// Count missed cleavages
    ///
    /// Reminder: Need to be a instance method because of the trait object, otherwise this could be a static method
    /// Therefore min_length, max_length and max_missed_cleavages can be `None` when just this method is used.
    ///
    fn count_missed_cleavages(&self, sequence: &str) -> Result<usize>;

    /// Cleaves a protein into peptides and returns a iterator over the peptides
    ///
    /// # Arguments
    /// * `sequence` - Amino acid sequence
    /// * `max_missed_cleavages` - Maximum number of missed cleavages
    ///
    fn cleave(&self, sequence: &str) -> Result<ProteaseIterator> {
        Ok(ProteaseIterator::new(
            self.full_digest(sequence)?,
            self.get_min_length(),
            self.get_max_length(),
            self.get_max_missed_cleavages(),
            self.is_count_missed_cleavages(),
        ))
    }
}

pub struct ProteaseIterator {
    /// Fully digested protein sequence
    full_digest: Vec<String>,
    /// Min peptide length
    min_length: Option<usize>,
    /// Max peptide length
    max_length: Option<usize>,
    /// Maximum number of missed cleavages
    max_missed_cleavages: Option<usize>,
    is_count_missed_cleavages: bool,
    /// Start position of the iterator
    start_position: usize,
    /// Buffer when ambiguous amino acids are resolved
    /// multiple peptides are returned and need to be stored temporarily
    peptide_buffer: Vec<Peptide>,
}

impl ProteaseIterator {
    pub fn new(
        full_digest: Vec<String>,
        min_length: Option<usize>,
        max_length: Option<usize>,
        max_missed_cleavages: Option<usize>,
        is_count_missed_cleavages: bool,
    ) -> Self {
        let initial_buffer_capacity = match max_missed_cleavages {
            Some(max_missed_cleavages) => max_missed_cleavages + 1,
            None => full_digest.len(),
        };
        Self {
            full_digest,
            min_length,
            max_length,
            max_missed_cleavages,
            is_count_missed_cleavages,
            start_position: 0,
            peptide_buffer: Vec::with_capacity(initial_buffer_capacity),
        }
    }
}

impl FallibleIterator for ProteaseIterator {
    type Item = Peptide;
    type Error = Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        loop {
            // First empty the buffer
            while let Some(peptide) = self.peptide_buffer.pop() {
                return Ok(Some(peptide));
            }
            // Check if the end of the full digest is reached
            if self.start_position > self.full_digest.len() {
                return Ok(None);
            }

            // Fill buffer

            // With a given missed cleavage limit the end position is the start position + the limit
            // or the end of the full digest whichever is nearer to avoid overflow
            // If however no limit is given the end position is the end of the full digest
            let end = match self.max_missed_cleavages {
                Some(max_missed_cleavages) => cmp::min(
                    self.start_position + max_missed_cleavages,
                    self.full_digest.len() - 1,
                ),
                None => self.full_digest.len() - 1,
            };

            for i in self.start_position..=end {
                let sequence = self.full_digest[self.start_position..=i].join("");
                let mut missed_cleavages = i - self.start_position;
                if let Some(min_length) = self.min_length {
                    if sequence.len() < min_length {
                        continue;
                    }
                }
                if let Some(max_length) = self.max_length {
                    if sequence.len() > max_length {
                        continue;
                    }
                }
                if let Some(max_missed_cleavages) = self.max_missed_cleavages {
                    if missed_cleavages > max_missed_cleavages {
                        continue;
                    }
                }
                // If the protease was not initialized with a limit for missed cleavages
                // the missed cleavages are set to 0
                missed_cleavages = if self.is_count_missed_cleavages {
                    missed_cleavages
                } else {
                    0
                };
                self.peptide_buffer
                    .push(Peptide::new(sequence, missed_cleavages)?);
            }

            // Increase start position
            self.start_position += 1;
        }
    }
}
