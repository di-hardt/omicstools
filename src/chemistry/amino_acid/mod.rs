/// (Non-) canconical amino acids
#[allow(clippy::module_inception)]
pub mod amino_acid;
/// Hydrophobicity scales
pub mod hydrophobicity;
/// Rexport amino acid module for FASTER access
pub use amino_acid::*;
