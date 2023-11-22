// # Create absolute path to readme ti increase compatible for different build targets
//  https://gist.github.com/JakeHartnell/2c1fa387f185f5dc46c9429470a2e2be
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Readme.md"))]

#[macro_use]
extern crate lazy_static;

/// Functions and extension for external crates
#[macro_use]
pub mod tools;

/// Module containing functions and information for biology
pub mod biology;
/// Module containing functions and information for chemistry and molecules
pub mod chemistry;
/// Entities and functions for working with mass spectrometry data
pub mod mass_spectrometry;
/// Module containing functions and definitions for proteomics
pub mod proteomics;
