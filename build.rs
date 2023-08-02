// std imports
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// 3rd party imports
use anyhow::Result;
use askama::Template;
use csv;

// Add support for type deserialization through `serde` when using `csv`-crate
type CanonicalAminoAcidRecord = (String, char, String, String, f64, f64);
type NonCanonicalAminoAcidRecord = (String, char, String, f64, f64);
type ElementRecord = (String, String, f64, f64);
type SubatomicParticleRecord = (String, f64);

/// Template for amino acids
///
#[derive(Template)]
#[template(path = "amino_acid.rs.jinja", escape = "none")]
struct AminoAcidTemplate {
    canonical_data: Vec<CanonicalAminoAcidRecord>,
    non_canonical_data: Vec<NonCanonicalAminoAcidRecord>,
}

/// Template for elements
///
#[derive(Template)]
#[template(path = "element.rs.jinja", escape = "none")]
struct ElementTemplate {
    data: Vec<ElementRecord>,
}

/// Template for subatomic particles
///
#[derive(Template)]
#[template(path = "subatomic_particle.rs.jinja", escape = "none")]
struct SubatomicParticleTemplate {
    data: Vec<SubatomicParticleRecord>,
}

/// Custom filters for askama templates
///
mod filters {
    pub fn len<T>(vec: &Vec<T>) -> askama::Result<usize> {
        Ok(vec.len())
    }

    pub fn upper_snake_case(string: &str) -> askama::Result<String> {
        Ok(string.replace(" ", "_").to_uppercase())
    }

    /// Prevents 0.0 to be shortened to 0
    ///
    pub fn f64_to_string(number: &f64) -> askama::Result<String> {
        Ok(format!("{:?}", number))
    }
}

/// Compiles / renders the amino acid template
///
/// # Arguments
/// * `out_dir` - The output directory
///
fn compile_amino_acids(out_dir: &str) -> Result<()> {
    let dest_path = Path::new(&out_dir).join("amino_acid.rs");
    let mut csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path("data/canonical_amino_acids.csv")?;

    // get canonical amino acid data
    let canonical_data = csv_reader
        .deserialize()
        .collect::<Result<Vec<CanonicalAminoAcidRecord>, _>>()?;

    let mut csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path("data/non_canonical_amino_acids.csv")?;

    // get non canonical amino acid data
    let non_canonical_data = csv_reader
        .deserialize()
        .collect::<Result<Vec<NonCanonicalAminoAcidRecord>, _>>()?;

    // render template
    let mut f = File::create(dest_path)?;
    let data = AminoAcidTemplate {
        canonical_data,
        non_canonical_data,
    };
    writeln!(f, "{}", data.render()?)?;

    Ok(())
}

/// Compiles / renders the elements template
///
/// # Arguments
/// * `out_dir` - The output directory
///
fn compile_elements(out_dir: &str) -> Result<()> {
    let dest_path = Path::new(&out_dir).join("element.rs");
    let mut csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path("data/elements.csv")?;

    // get element data
    let data = csv_reader
        .deserialize()
        .collect::<Result<Vec<ElementRecord>, _>>()?;

    // render template
    let mut f = File::create(dest_path)?;
    let data = ElementTemplate { data };
    writeln!(f, "{}", data.render()?)?;

    Ok(())
}

fn compile_subatomic_particles(out_dir: &str) -> Result<()> {
    let dest_path = Path::new(&out_dir).join("subatomic_particle.rs");
    let mut csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path("data/subatomic_particles.csv")?;

    // get subatomic particle data
    let data = csv_reader
        .deserialize()
        .collect::<Result<Vec<SubatomicParticleRecord>, _>>()?;

    // render template
    let mut f = File::create(dest_path)?;
    let data = SubatomicParticleTemplate { data };
    writeln!(f, "{}", data.render()?)?;

    Ok(())
}

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;

    compile_amino_acids(out_dir.as_str())?;
    compile_elements(out_dir.as_str())?;
    compile_subatomic_particles(out_dir.as_str())?;

    // Setup instructions
    println!("cargo:rerun-if-changed=data/canonical_amino_acids.csv");
    println!("cargo:rerun-if-changed=data/non_canonical_amino_acids.csv");
    println!("cargo:rerun-if-changed=data/elements.csv");
    println!("cargo:rerun-if-changed=data/subatomic_particles.csv");

    Ok(())
}
