/// I/O for various proteomics data formats   
/// Note: Most operation on file content is working with byte representation not strings as this get rid of any encoding/decoding issues and overhead(?).
pub mod io;
pub mod ontology;
/// Peptide definition
pub mod peptide;
/// Defines posttranslational modifications
pub mod post_translational_modifications;
/// Defines proteases
pub mod proteases;
