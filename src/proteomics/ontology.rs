use std::{collections::HashSet, sync::OnceLock};

use anyhow::{anyhow, Context, Result};
use fastobo_graphs::IntoGraph;

/// The subfolder in the home directory where the ontology files are cached.
///
const ONTOLOGY_FOLDER: &str = ".life_science_ontologies";

/// Access ontology as needed.
/// Data is only fetched once and cached in the home directory.
///
pub struct Ontology {
    pub name: &'static str,
    pub url: &'static str,
    pub filename: &'static str,
    pub graph: OnceLock<Result<fastobo_graphs::model::GraphDocument>>,
}

impl Ontology {
    /// Downloads the ontology from the given URL.
    ///
    fn download(&self) -> Result<String> {
        let response = reqwest::blocking::get(self.url).map_err(|err| {
            anyhow!(
                "Error fetch fetching {} from {}: {}",
                self.name,
                self.url,
                err
            )
        })?;
        response
            .text()
            .map_err(|err| anyhow!("Error reading response from {}: {}", self.url, err))
    }

    /// Returns the ontology as a fastobo graph.
    /// Internet connection is required.
    ///
    pub fn get_graph(&self) -> &Result<fastobo_graphs::model::GraphDocument> {
        self.graph.get_or_init(|| {
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

                let ontology_path = cache_path.join(self.filename);
                if !ontology_path.is_file() {
                    let plain_ontology = self.download()?;
                    std::fs::write(&ontology_path, plain_ontology).map_err(|err| {
                        anyhow!("Error write {} to {}: {}", self.filename, self.url, err)
                    })?;
                }
                let doc = fastobo::from_file(ontology_path).map_err(|err| {
                    anyhow!("Error reading {} from {}: {}", self.filename, self.url, err)
                })?;
                return Ok(doc.into_graph()?);
            }

            let plain_ontology = self.download()?;

            let doc = fastobo::from_str(plain_ontology).map_err(|err| {
                anyhow!("Error reading {} from {}: {}", self.filename, self.url, err)
            })?;
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
                let children = PSI_MS.collect_children_associations(accession)?;
                Ok(children)
            }
            "UO" => {
                let children = UNIT.collect_children_associations(accession)?;
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
    /// * `accession` - The accession of the term.
    ///
    fn collect_children_associations(&self, accession: &str) -> Result<Vec<String>> {
        let url_accession = accession.replace(":", "_");
        // Get inner data
        let graph = match self.get_graph().as_ref() {
            Ok(data) => data,
            Err(e) => return Err(anyhow::anyhow!("Error loading ontology: {}", e)),
        };

        let mut children: Vec<String> = graph
            .graphs
            .iter()
            .flat_map(|g| {
                g.edges
                    .iter()
                    .filter(|e| e.obj.ends_with(&url_accession) && e.pred.ends_with("is_a"))
                    .map(|e| match e.sub.split("/").last() {
                        Some(child) => {
                            let new_child = child.replace("_", ":");
                            let mut next_level_children = Ontology::get_children_of(&new_child)?;
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
}

/// The PSI-MS ontology.
///
pub static PSI_MS: Ontology = Ontology {
    name: "PSI-MS",
    url: "https://raw.githubusercontent.com/HUPO-PSI/psi-ms-CV/refs/tags/v4.1.184/psi-ms.obo",
    filename: "psi-ms.obo",
    graph: OnceLock::new(),
};

/// The Unit ontology.
///
pub static UNIT: Ontology = Ontology {
    name: "Unit",
    url: "https://raw.githubusercontent.com/bio-ontology-research-group/unit-ontology/refs/tags/v2023-05-23/unit-ontology-full.obo",
    filename: "uo-full.obo",
    graph: OnceLock::new(),
};

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

    /// The expected children of the term with the accession `UO:1000010`.
    ///
    const EXPECTED_CHILDREN_FOR_UNIT_TEST: [&str; 5] = [
        "UO:0000029",
        "UO:0000028",
        "UO:0000150",
        "UO:0000030",
        "UO:0000010",
    ];

    #[test]
    fn test_get_psi_ms() {
        let children = Ontology::get_children_of("MS:1000044").unwrap();

        assert_eq!(children.len(), EXPECTED_CHILDREN_FOR_PSI_MS_TEST.len());
        for expected_child in &EXPECTED_CHILDREN_FOR_PSI_MS_TEST {
            assert!(children.contains(&expected_child.to_string()));
        }
    }

    #[test]
    fn test_get_unit() {
        let children = Ontology::get_children_of("UO:1000010").unwrap();

        assert_eq!(children.len(), EXPECTED_CHILDREN_FOR_UNIT_TEST.len());
        for expected_child in &EXPECTED_CHILDREN_FOR_UNIT_TEST {
            assert!(children.contains(&expected_child.to_string()));
        }
    }
}
