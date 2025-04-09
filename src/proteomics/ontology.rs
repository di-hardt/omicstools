use std::{collections::HashSet, sync::OnceLock};

use anyhow::{Context, Result};
use fastobo_graphs::IntoGraph;

const ONTOLOGY_FOLDER: &str = ".life_science_ontologies";

/// The URL of the PSI-MS ontology.
///
const PSI_MS_URL: &str =
    "https://raw.githubusercontent.com/HUPO-PSI/psi-ms-CV/refs/tags/v4.1.184/psi-ms.obo";

lazy_static! {
    /// The PSI-MS ontology as a fastobo graph.
    static ref PSI_MS: OnceLock<Result<fastobo_graphs::model::GraphDocument>> = OnceLock::new();
}

/// Returns the PSI-MS ontology as a fastobo graph.
/// Internet connection is required.
///
pub fn get_psi_ms() -> &'static Result<fastobo_graphs::model::GraphDocument> {
    PSI_MS.get_or_init(|| {
        // Check if the home directory is available, if so set cache path
        #[allow(deprecated)] // deprecation flag is supposed to be removed in 1.86 for home_dir
        let cache_path = match std::env::home_dir() {
            Some(path) => {
                if path.is_dir() {
                    Some(path.join(ONTOLOGY_FOLDER))
                } else {
                    None
                }
            }
            None => None,
        };

        // Check if the cache path is set and create it if it doesn't exist
        if let Some(cache_path) = cache_path {
            if !cache_path.is_dir() {
                std::fs::create_dir_all(&cache_path)
                    .context("Error creating ontology cache directory")?;
            }

            let ontology_path = cache_path.join("psi-ms.obo");
            if !ontology_path.is_file() {
                println!("Downloading PSI-MS ontology to {}", ontology_path.display());
                let response =
                    reqwest::blocking::get(PSI_MS_URL).context("Error fetching PSI-MS ontology")?;
                std::fs::write(&ontology_path, response.bytes()?)
                    .context("Error writing PSI-MS ontology to file")?;
            }
            println!("Reading PSI-MS ontology from {}", ontology_path.display());
            let doc = fastobo::from_file(ontology_path).context("Error reading PSI-MS ontology")?;
            return Ok(doc.into_graph()?);
        }

        let response =
            reqwest::blocking::get(PSI_MS_URL).context("Error fetching PSI-MS ontology")?;

        let doc = fastobo::from_str(response.text()?).context("Error reading PSI-MS ontology")?;
        Ok(doc.into_graph()?)
    })
}

/// Returns the accession of all the children of the term with the given accession.
///
/// # Arguments
/// * `accession` - The accession of the term.
///
pub fn get_children_of(accession: &str) -> Result<Vec<String>> {
    let colon_pos = match accession.find(':') {
        Some(pos) => pos,
        None => {
            return Err(anyhow::anyhow!(
                "Invalid accession `{}`, has not ontology part",
                accession
            ))
        }
    };

    match &accession[..colon_pos] {
        "MS" => {
            let psi_ms = match get_psi_ms().as_ref() {
                Ok(psi_ms) => psi_ms,
                Err(e) => return Err(anyhow::anyhow!("Error loading PSI-MS ontology: {}", e)),
            };
            let children = collect_children_associations(psi_ms, accession)?;
            Ok(children)
        }
        _ => Err(anyhow::anyhow!(
            "Invalid ontology part `{}` in accession `{}`",
            &accession[..colon_pos],
            accession
        )),
    }
}

/// Collects the children associations of the term with the given accession.
///
/// # Arguments
/// * `ontology` - The ontology graph.
/// * `accession` - The accession of the term.
///
fn collect_children_associations(
    ontology: &fastobo_graphs::model::GraphDocument,
    accession: &str,
) -> Result<Vec<String>> {
    let url_accession = accession.replace(":", "_");
    let mut children: Vec<String> = ontology
        .graphs
        .iter()
        .flat_map(|g| {
            g.edges
                .iter()
                .filter(|e| e.obj.ends_with(&url_accession) && e.pred.ends_with("is_a"))
                .map(|e| match e.sub.split("/").last() {
                    Some(child) => {
                        let new_child = child.replace("_", ":");
                        let mut next_level_children = get_children_of(&new_child)?;
                        next_level_children.push(new_child);
                        Ok(next_level_children)
                    }
                    None => Err(anyhow::anyhow!(
                        "Error parsing child accession from `{:?}`",
                        e
                    )),
                })
        })
        .collect::<Result<Vec<Vec<String>>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<String>>();

    let unique_childrens: HashSet<String> = children.drain(..).collect();
    children.extend(unique_childrens);
    Ok(children)
}

#[cfg(test)]
mod tests {

    use super::*;

    /// The expected children of the term with the accession `MS:1000044`.
    /// ToDo: check for something more reliable
    ///
    const EXPECTED_CHILDREN_FOR_PSI_MS_TEST: [&str; 22] = [
        "MS:1000133",
        "MS:1000134",
        "MS:1000135",
        "MS:1000136",
        "MS:1000242",
        "MS:1000250",
        "MS:1000262",
        "MS:1000282",
        "MS:1000422",
        "MS:1000433",
        "MS:1000435",
        "MS:1000598",
        "MS:1000599",
        "MS:1001880",
        "MS:1002000",
        "MS:1002472",
        "MS:1002481",
        "MS:1002678",
        "MS:1002679",
        "MS:1003246",
        "MS:1003247",
        "MS:1003294",
    ];

    #[test]
    fn test_get_psi_ms() {
        let children = get_children_of("MS:1000044").unwrap();

        assert_eq!(children.len(), EXPECTED_CHILDREN_FOR_PSI_MS_TEST.len());
        for expected_child in &EXPECTED_CHILDREN_FOR_PSI_MS_TEST {
            assert!(children.contains(&expected_child.to_string()));
        }
    }
}
