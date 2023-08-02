// # Create absolute path to readme ti increase compatible for different build targets
//  https://gist.github.com/JakeHartnell/2c1fa387f185f5dc46c9429470a2e2be
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Readme.md"))]

/// Module containing functions and information for chemistry and molecules
pub mod chemistry;
/// Module containing functions and definitions for proteomics
pub mod proteomics;
