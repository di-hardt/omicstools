use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{index::Index, is_element::IsElement};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "index")]
    pub indexes: Vec<Index>,
}

impl<'a> IndexList {
    pub fn get_index_by_name(&'a self, name: &str) -> Option<&'a Index> {
        self.indexes.iter().find(|index| index.name == name)
    }
}

impl IsElement for IndexList {
    fn validate(&self) -> Result<()> {
        for index in &self.indexes {
            index.validate()?;
        }
        Ok(())
    }
}
