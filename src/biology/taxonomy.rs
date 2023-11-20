// std imports
use std::collections::HashMap;

// 3rd party imports
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::Dfs;
use serde::{Deserialize, Serialize};

/// A taxonomy as noted in NCBI's node tree
/// Not all attributes are used in this crate, e.g. only the scientific name is used
///
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Taxonomy {
    /// Taxonomy ID
    id: u64,
    /// Parent taxonomy ID
    parent_id: u64,
    /// Scientific name
    scientific_name: String,
    // internal rank ID, can be looked up in the ranks HashMap in TaxonomyTree
    rank_id: u64,
}

impl Taxonomy {
    /// Creates a new taxonomy
    ///
    /// # Arguments
    /// * `id` - The taxonomy ID
    /// * `parent_id` - The parent taxonomy ID
    /// * `scientific_name` - The scientific name
    /// * `rank_id` - The rank ID
    ///
    pub fn new(id: u64, parent_id: u64, scientific_name: String, rank_id: u64) -> Self {
        Taxonomy {
            id,
            parent_id,
            scientific_name,
            rank_id,
        }
    }

    /// Returns the taxonomy ID
    ///
    pub fn get_id(&self) -> u64 {
        self.id
    }

    /// Returns the parent taxonomy ID
    ///
    pub fn get_parent_id(&self) -> u64 {
        self.parent_id
    }

    /// Returns the scientific name
    ///
    pub fn get_scientific_name(&self) -> &str {
        self.scientific_name.as_str()
    }

    /// Returns the internal rank ID
    /// Can be looked up in the ranks HashMap in TaxonomyTree
    ///
    pub fn get_rank_id(&self) -> u64 {
        self.rank_id
    }
}

/// The taxonomy tree as noted in NCBI's node tree
///
#[derive(Serialize, Deserialize)]
pub struct TaxonomyTree {
    /// Map between internal rank ID and rank name
    /// makes it more efficient than storing the rank name in each taxonomy
    ranks: HashMap<u64, String>,
    /// Map between taxonomy ID and tree node
    index: HashMap<u64, NodeIndex>,
    /// The taxonomy tree
    tree: Graph<Taxonomy, ()>,
    /// Map of merged taxonomies old => new
    merged_taxonomies: HashMap<u64, u64>,
    /// IDs of deleted taxonomies
    deleted_taxonomies: Vec<u64>,
}

impl TaxonomyTree {
    pub fn new(
        ranks: HashMap<u64, String>,
        taxonomies: Vec<Taxonomy>,
        merged_taxonomies: HashMap<u64, u64>,
        deleted_taxonomies: Vec<u64>,
    ) -> TaxonomyTree {
        let mut tree = Graph::<Taxonomy, ()>::new();
        let mut index: HashMap<u64, NodeIndex> = HashMap::with_capacity(taxonomies.len());
        for taxonomy in taxonomies {
            let tax_id = taxonomy.get_id();
            let node = tree.add_node(taxonomy);
            index.insert(tax_id, node);
        }

        for (_, tax_node) in index.iter() {
            let taxonomy = &tree[*tax_node];
            let parent_node = index.get(&taxonomy.get_parent_id()).unwrap();
            tree.add_edge(*parent_node, *tax_node, ());
        }

        TaxonomyTree {
            ranks,
            index,
            tree,
            merged_taxonomies,
            deleted_taxonomies,
        }
    }

    /// Returns the ID/rank mal
    ///
    pub fn get_ranks(&self) -> &HashMap<u64, String> {
        &self.ranks
    }

    /// Returns the rank names as a vector
    ///
    pub fn get_rank_names(&self) -> Vec<&String> {
        self.ranks.values().collect()
    }

    /// Returns the rank name for the given rank ID
    ///
    pub fn get_rank(&self, rank_id: u64) -> Option<&String> {
        self.ranks.get(&rank_id)
    }

    /// Returns the inner taxonomy tree
    ///
    pub fn get_tree(&self) -> &Graph<Taxonomy, ()> {
        &self.tree
    }

    /// Returns the index of taxonomy IDs to tree nodes
    ///
    pub fn get_index(&self) -> &HashMap<u64, NodeIndex> {
        &self.index
    }

    /// Returns a map of merged taxonomies
    /// The key is the old taxonomy ID, the value is the new taxonomy ID
    ///
    pub fn get_merged_taxonomies(&self) -> &HashMap<u64, u64> {
        &self.merged_taxonomies
    }

    /// Returns the IDs of deleted taxonomies
    ///
    pub fn get_deleted_taxonomies(&self) -> &Vec<u64> {
        &self.deleted_taxonomies
    }

    /// Returns the taxonomy node for the given taxonomy ID
    /// If the ID was not found in index, it will be looked up in the merged taxonomies as well
    ///
    /// # Arguments
    /// * `tax_id` - The taxonomy ID
    ///
    pub fn find_taxonomy_node(&self, tax_id: u64) -> Option<NodeIndex> {
        match self.index.get(&tax_id) {
            Some(tax_node) => Some(*tax_node),
            None => {
                let merged_id = self.merged_taxonomies.get(&tax_id)?;
                Some(*self.index.get(merged_id)?)
            }
        }
    }

    pub fn get_taxonomies(&self) -> Vec<&Taxonomy> {
        self.tree.node_indices().map(|i| &self.tree[i]).collect()
    }

    /// Returns the taxonomy with the given ID
    ///
    /// # Arguments
    /// * `tax_id` - The taxonomy ID
    ///
    pub fn get_taxonomy(&self, tax_id: u64) -> Option<&Taxonomy> {
        Some(&self.tree[self.find_taxonomy_node(tax_id)?])
    }

    /// Returns the given taxonomy and all its sub-taxonomies
    ///
    /// # Arguments
    /// * `id` - The taxonomy id
    ///
    pub fn get_sub_taxonomies(&self, tax_id: u64) -> Option<Vec<&Taxonomy>> {
        let tax_node = self.find_taxonomy_node(tax_id)?;
        let start_taxonomy = &self.tree[tax_node];
        let mut sub_taxonomies: Vec<&Taxonomy> = Vec::new();
        let mut dfs = Dfs::new(&self.tree, tax_node);
        while let Some(node) = dfs.next(&self.tree) {
            let taxonomy = &self.tree[node];
            if taxonomy.get_id() != start_taxonomy.get_id() {
                sub_taxonomies.push(taxonomy);
            }
        }
        Some(sub_taxonomies)
    }
}

#[cfg(test)]
mod tests {
    // 3rd party imports
    use serde_json;

    // internal imports
    use super::*;
    use crate::biology::io::taxonomy_reader::tests::get_taxdmp_zip;
    use crate::biology::io::taxonomy_reader::TaxonomyReader;

    /// Make sure after serializing and deserializing the NodeIndexes and Taxonomy/NodeIndex map
    /// are still valid and the
    #[test]
    fn test_serialization() {
        let taxdmp_zip_path = get_taxdmp_zip().unwrap();

        let taxonomy_tree = TaxonomyReader::new(&taxdmp_zip_path)
            .unwrap()
            .read()
            .unwrap();

        let tree_json = serde_json::to_string(&taxonomy_tree).unwrap();
        let tree_deserialized: TaxonomyTree = serde_json::from_str(&tree_json).unwrap();
        assert_eq!(taxonomy_tree.get_ranks(), tree_deserialized.get_ranks());
        assert_eq!(taxonomy_tree.get_index(), tree_deserialized.get_index());
        assert_eq!(
            taxonomy_tree.get_merged_taxonomies(),
            tree_deserialized.get_merged_taxonomies()
        );
        assert_eq!(
            taxonomy_tree.get_deleted_taxonomies(),
            tree_deserialized.get_deleted_taxonomies()
        );
        // Go through all taxonomies and check if the same taxonomy is returned
        // by ID and if both have the same parent ID
        for tax_id in taxonomy_tree.get_index().keys() {
            let taxonomy = taxonomy_tree.get_taxonomy(*tax_id).unwrap();
            let taxonomy_deserialized = tree_deserialized.get_taxonomy(*tax_id).unwrap();
            // Check if the same taxonomy is returned by ID
            assert_eq!(taxonomy, taxonomy_deserialized);
            // Get the nodes
            let taxonomy_node = taxonomy_tree.find_taxonomy_node(*tax_id).unwrap();
            let taxonomy_node_deserialized = tree_deserialized.find_taxonomy_node(*tax_id).unwrap();
            // Get the parent nodes assert one parent only
            let taxonomy_parent_nodes: Vec<NodeIndex> = taxonomy_tree
                .get_tree()
                .neighbors_directed(taxonomy_node, petgraph::Incoming)
                .collect();
            let taxonomy_deserialized_parent_nodes: Vec<NodeIndex> = tree_deserialized
                .get_tree()
                .neighbors_directed(taxonomy_node_deserialized, petgraph::Incoming)
                .collect();
            assert_eq!(taxonomy_parent_nodes.len(), 1);
            assert_eq!(taxonomy_deserialized_parent_nodes.len(), 1);
            // Get parent taxonomies asserting they are the same
            let taxonomy_parent = &taxonomy_tree.get_tree()[taxonomy_parent_nodes[0]];
            let taxonomy_deserialized_parent =
                &tree_deserialized.get_tree()[taxonomy_deserialized_parent_nodes[0]];
            assert_eq!(taxonomy_parent, taxonomy_deserialized_parent);
        }
    }
}
