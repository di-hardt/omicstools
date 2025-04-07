use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{file_content::FileContent, is_element::IsElement, source_file_list::SourceFileList};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDescription {
    #[serde(rename = "fileContent")]
    pub file_content: FileContent,
    #[serde(rename = "sourceFileList")]
    pub source_file_list: SourceFileList,
}

impl IsElement for FileDescription {
    fn validate(&self) -> Result<()> {
        self.file_content.validate()?;
        self.source_file_list.validate()?;
        Ok(())
    }
}
