use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{is_element::IsElement, is_list::IsList, source_file::SourceFile};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceFileList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "sourceFile")]
    pub source_files: Vec<SourceFile>,
}

impl IsElement for SourceFileList {
    fn validate(&self) -> Result<()> {
        if self.count != self.source_files.len() {
            bail!(
                "The count attribute ({}) does not match the number of sourceFile elements ({})",
                self.count,
                self.source_files.len()
            );
        }

        for source_file in &self.source_files {
            source_file.validate()?;
        }
        Ok(())
    }
}

impl IsList<'_, SourceFile> for SourceFileList {
    fn iter(&self) -> std::slice::Iter<'_, SourceFile> {
        self.source_files.iter()
    }
}
