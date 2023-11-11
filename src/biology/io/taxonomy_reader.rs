// std imports
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

// 3rd party imports
use anyhow::Result;
use anyhow::{bail, Context};
use zip;

// internal imports
use crate::biology::taxonomy::{Taxonomy, TaxonomyTree};

/// URL of the latest `taxdmp.zip` file
///
pub const TAXDMP_URL: &'static str = "https://ftp.ncbi.nih.gov/pub/taxonomy/taxdmp.zip";

/// Creates a [TaxonomyTree](create::biology::taxonomy::TaxonomyTree) from a `taxdmp.zip` file
/// downloaded from [NCBI taxonomy](https://ftp.ncbi.nih.gov/pub/taxonomy/).
///
pub struct TaxonomyReader {
    archive: zip::ZipArchive<BufReader<File>>,
}

impl TaxonomyReader {
    pub fn new(zip_file_path: &Path) -> Result<TaxonomyReader> {
        let file = File::open(zip_file_path)?;
        let reader = BufReader::new(file);
        let archive = zip::ZipArchive::new(reader)?;
        Ok(TaxonomyReader { archive })
    }

    pub fn read(&mut self) -> Result<TaxonomyTree> {
        let mut deleted_taxonomies: Vec<u64> = Vec::new();
        let mut merged_taxonomies: HashMap<u64, u64> = HashMap::new();
        let mut name_map: HashMap<u64, String> = HashMap::new();
        let mut ranks: HashMap<u64, String> = HashMap::new();
        let mut taxonomies: Vec<(u64, u64, String, u64)> = Vec::new();

        for i in 0..self.archive.len() {
            let mut file = self.archive.by_index(i).unwrap();
            match file.name() {
                "delnodes.dmp" => {
                    deleted_taxonomies = Self::read_delnodes(&mut file)?;
                }
                "merged.dmp" => {
                    merged_taxonomies = Self::read_merged(&mut file)?;
                }
                "names.dmp" => {
                    name_map = Self::read_names(&mut file)?;
                }
                "nodes.dmp" => {
                    (ranks, taxonomies) = Self::read_nodes(&mut file)?;
                }
                _ => {}
            }
        }
        let taxonomies: Vec<Taxonomy> = taxonomies
            .iter()
            .map(|(tax_id, parent_id, _, rank)| {
                let name = match name_map.get(tax_id) {
                    Some(n) => n,
                    None => bail!("No name found for tax_id {}", tax_id),
                };
                Ok(Taxonomy::new(*tax_id, *parent_id, name.clone(), *rank))
            })
            .collect::<Result<Vec<Taxonomy>>>()?;

        Ok(TaxonomyTree::new(
            ranks,
            taxonomies,
            merged_taxonomies,
            deleted_taxonomies,
        ))
    }

    fn read_delnodes(file: &mut zip::read::ZipFile) -> Result<Vec<u64>> {
        let reader = BufReader::new(file);
        Ok(reader
            .lines()
            .map(|line| {
                Ok(line?
                    .replace("|", "")
                    .trim()
                    .parse::<u64>()
                    .context("Error when parsing taxonomy ID in delnodes.dmp")?)
            })
            .collect::<Result<Vec<u64>>>()?)
    }

    fn read_merged(file: &mut zip::read::ZipFile) -> Result<HashMap<u64, u64>> {
        let reader = BufReader::new(file);
        Ok(reader
            .lines()
            .map(|line| {
                let line = line?;
                let mut split = line.split("|");
                let old_id = split
                    .next()
                    .unwrap()
                    .trim()
                    .parse::<u64>()
                    .context("Error when parsing old taxonomy id in merged.dmp")?;
                let new_id = split
                    .next()
                    .unwrap()
                    .trim()
                    .parse::<u64>()
                    .context("Error when parsing new taxonomy id in merged.dmp")?;
                Ok((old_id, new_id))
            })
            .collect::<Result<HashMap<u64, u64>>>()?)
    }

    fn read_nodes(
        file: &mut zip::read::ZipFile,
    ) -> Result<(HashMap<u64, String>, Vec<(u64, u64, String, u64)>)> {
        let mut ranks: HashMap<String, u64> = HashMap::new();
        let mut taxonomies: Vec<(u64, u64, String, u64)> = Vec::new();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let mut split = line.split("|");
            let tax_id = split
                .next()
                .unwrap()
                .trim()
                .parse::<u64>()
                .context("Error when parsing taxonomy ID in nodes.dmp")?;
            let parent_id = split
                .next()
                .unwrap()
                .trim()
                .parse::<u64>()
                .context("Error when parsing parent taxonomy ID in nodes.dmp")?;
            let rank = split.next().unwrap().trim().to_string();
            let rank_id = match ranks.get(&rank) {
                Some(r) => *r,
                None => {
                    ranks.insert(rank, ranks.len() as u64);
                    ranks.len() as u64 - 1
                }
            };
            taxonomies.push((tax_id, parent_id, String::new(), rank_id));
        }
        // Switch rank name and ID
        let ranks = ranks
            .into_iter()
            .map(|(rank, rank_id)| (rank_id, rank))
            .collect::<HashMap<u64, String>>();
        Ok((ranks, taxonomies))
    }

    fn read_names(file: &mut zip::read::ZipFile) -> Result<HashMap<u64, String>> {
        let reader = BufReader::new(file);
        let mut name_maps: HashMap<u64, String> = HashMap::new();
        for line in reader.lines() {
            let line = line?;
            let mut split = line.split("|");
            let tax_id = split
                .next()
                .unwrap()
                .trim()
                .parse::<u64>()
                .context("Error when parsing taxonomy ID in names.dmp")?;
            let name = split.next().unwrap();
            split.next().unwrap().trim().to_string();
            let name_class = split.next().unwrap().trim().to_string();
            if name_class == "scientific name" {
                name_maps.insert(tax_id, name.trim().to_string());
            }
        }
        Ok(name_maps)
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashSet;
    // std imports
    use std::env;
    use std::fs::create_dir;
    use std::io::Cursor;
    use std::path::PathBuf;
    use std::time::Duration;

    // 3rd party imports
    use anyhow::Result;
    use petgraph::visit::Bfs;
    use reqwest::header::USER_AGENT;

    // internal imports
    use super::*;

    /// Homo sapiens neanderthalensis & Homo sapiens subsp. 'Denisova'
    const EXPECTED_HOMO_SAPIENS_SUB_TREE_IDS: [u64; 2] = [63221, 741158];

    /// Tests if all lines are parsed from each dmp files are present.
    /// Tree structure is only validated in terms that each taxonomy is only present once.
    /// But with validation of nodes it should make sure everything is present.
    /// Note: `names.dmp` is not validated as it requires again parsing of each line file.
    ///
    #[test]
    fn test_read() {
        let taxdmp_zip_path = get_taxdmp_zip().unwrap();

        let mut reader = TaxonomyReader::new(&taxdmp_zip_path).unwrap();
        let tree = reader.read().unwrap();

        let taxdmp_zip_file = File::open(&taxdmp_zip_path).unwrap();
        let zip_reader = BufReader::new(taxdmp_zip_file);
        let mut archive = zip::ZipArchive::new(zip_reader).unwrap();

        for i in 0..archive.len() {
            let mut dmp_file = archive.by_index(i).unwrap();
            match dmp_file.name() {
                "delnodes.dmp" => validate_delnodes(&tree, &mut dmp_file),
                "merged.dmp" => validate_merged(&tree, &mut dmp_file),
                "nodes.dmp" => validate_nodes(&tree, &mut dmp_file),
                _ => {}
            }
        }

        validate_uniqueness_of_taxonomy_nodes(&tree);

        // This may not make sense in the future as the tree might change
        let homo_sapiens_sub_tree = tree.get_sub_taxonomies(9606).unwrap();
        assert_eq!(homo_sapiens_sub_tree.len(), 2);
        for tax in homo_sapiens_sub_tree.iter() {
            assert!(EXPECTED_HOMO_SAPIENS_SUB_TREE_IDS.contains(&tax.get_id()));
        }
    }

    /// Downloads the latest taxdmp.zip if not given by env var
    /// `TAXDMP_ZIP_PATH` or already downloaded.
    ///
    pub fn get_taxdmp_zip() -> Result<PathBuf> {
        if let Some(taxdmp_zip_path) = env::var_os("TAXDMP_ZIP_PATH") {
            return Ok(Path::new(taxdmp_zip_path.to_str().unwrap()).to_path_buf());
        }
        let taxdmp_zip_path = env::temp_dir()
            .join("mcccoys_unit_tests")
            .join("taxdmp.zip");
        // Create temp dir
        if !taxdmp_zip_path.parent().unwrap().is_dir() {
            create_dir(taxdmp_zip_path.parent().unwrap()).unwrap();
        }
        // Avoid unnecessary downloads
        if taxdmp_zip_path.is_file() {
            return Ok(taxdmp_zip_path);
        }
        let client = reqwest::blocking::Client::new();
        // Create a request with custom user agent to apologies for download overhead
        let request = client
            .get(TAXDMP_URL)
            .header(
                USER_AGENT,
                format!(
                    "Unit test from '{}' here, apologies for the download overhead.",
                    env!("CARGO_PKG_REPOSITORY")
                ),
            )
            .timeout(Duration::from_secs(300));
        let response = request.send().unwrap();
        let mut file = std::fs::File::create(&taxdmp_zip_path)?;
        let mut content = Cursor::new(response.bytes().unwrap());
        std::io::copy(&mut content, &mut file)?;
        Ok(taxdmp_zip_path)
    }

    /// Just checks if the number of lines in the delnodes.dmp file is equal to the number of
    /// deleted taxonomies in the tree.
    fn validate_delnodes(tree: &TaxonomyTree, dmp_file: &mut zip::read::ZipFile) {
        let reader = BufReader::new(dmp_file);
        let num_lines = reader.lines().count();
        assert_eq!(num_lines, tree.get_deleted_taxonomies().len());
    }

    /// Just checks if the number of lines in the merged.dmp file is equal to the number of
    /// merged taxonomies in the tree.
    fn validate_merged(tree: &TaxonomyTree, dmp_file: &mut zip::read::ZipFile) {
        let reader = BufReader::new(dmp_file);
        let num_lines = reader.lines().count();
        assert_eq!(num_lines, tree.get_merged_taxonomies().len());
    }

    /// Just checks if the number of lines in the names.dmp file is equal to the number of
    /// taxonomies in the tree.
    fn validate_nodes(tree: &TaxonomyTree, dmp_file: &mut zip::read::ZipFile) {
        let reader = BufReader::new(dmp_file);
        let num_lines = reader.lines().count();
        assert_eq!(num_lines, tree.get_taxonomies().len());
    }

    /// Validates that each taxonomy is only present once in the tree.
    ///
    fn validate_uniqueness_of_taxonomy_nodes(tree: &TaxonomyTree) {
        let mut bfs = Bfs::new(&tree.get_tree(), tree.find_taxonomy_node(1).unwrap());
        let mut tax_ids: HashSet<u64> = HashSet::new();
        while let Some(node) = bfs.next(&tree.get_tree()) {
            let tax_id = tree.get_tree()[node].get_id();
            assert!(!tax_ids.contains(&tax_id));
            tax_ids.insert(tax_id);
        }
    }
}
